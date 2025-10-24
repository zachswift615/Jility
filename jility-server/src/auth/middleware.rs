use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jility_core::{ApiKey, Session, User};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::state::AppState;

use super::service::AuthService;

/// Authenticated user information that gets added to request extensions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
}

impl From<jility_core::UserModel> for AuthUser {
    fn from(user: jility_core::UserModel) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            full_name: user.full_name,
        }
    }
}

/// Authentication middleware that validates JWT tokens or API keys
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let user = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            validate_jwt_token(token, &state).await?
        }
        Some(header) if header.starts_with("ApiKey ") => {
            let key = &header[7..];
            validate_api_key(key, &state.db).await?
        }
        _ => {
            return Err(AppError::Unauthorized(
                "Missing or invalid authorization header".to_string(),
            ))
        }
    };

    // Add user to request extensions
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

/// Validate JWT token and return user
async fn validate_jwt_token(token: &str, state: &AppState) -> Result<AuthUser, AppError> {
    // Validate JWT
    let claims = state
        .auth_service
        .validate_jwt(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    // Parse user ID
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Check if session is revoked
    let token_hash = state
        .auth_service
        .hash_token(token)
        .map_err(|e| AppError::Internal(format!("Failed to hash token: {}", e)))?;

    let session = Session::find()
        .filter(jility_core::session::Column::TokenHash.eq(&token_hash))
        .filter(jility_core::session::Column::UserId.eq(user_id))
        .one(&**state.db)
        .await
        .map_err(|e| AppError::Database(e))?;

    if let Some(session) = session {
        if session.revoked_at.is_some() {
            return Err(AppError::Unauthorized("Token has been revoked".to_string()));
        }

        // Check if session expired
        if session.expires_at < chrono::Utc::now() {
            return Err(AppError::Unauthorized("Token has expired".to_string()));
        }
    } else {
        return Err(AppError::Unauthorized("Invalid session".to_string()));
    }

    // Fetch user from database
    let user = User::find_by_id(user_id)
        .one(&**state.db)
        .await
        .map_err(|e| AppError::Database(e))?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Unauthorized(
            "User account is inactive".to_string(),
        ));
    }

    Ok(user.into())
}

/// Validate API key and return user
async fn validate_api_key(
    key: &str,
    db: &Arc<DatabaseConnection>,
) -> Result<AuthUser, AppError> {
    // Get key prefix
    let prefix = AuthService::get_key_prefix(key)
        .map_err(|e| AppError::Unauthorized(format!("Invalid API key format: {}", e)))?;

    // Find API key by prefix
    let api_keys = ApiKey::find()
        .filter(jility_core::api_key::Column::Prefix.eq(&prefix))
        .all(&**db)
        .await
        .map_err(|e| AppError::Database(e))?;

    // Try to find matching key
    let auth_service = AuthService::new("".to_string()); // Only used for verification
    let mut matched_key: Option<jility_core::ApiKeyModel> = None;

    for api_key in api_keys {
        if auth_service.verify_password(key, &api_key.key_hash).unwrap_or(false) {
            matched_key = Some(api_key);
            break;
        }
    }

    let api_key = matched_key.ok_or_else(|| AppError::Unauthorized("Invalid API key".to_string()))?;

    // Check if key is revoked
    if api_key.revoked_at.is_some() {
        return Err(AppError::Unauthorized("API key has been revoked".to_string()));
    }

    // Check if key is expired
    if let Some(expires_at) = api_key.expires_at {
        if expires_at < chrono::Utc::now() {
            return Err(AppError::Unauthorized("API key has expired".to_string()));
        }
    }

    // Update last_used_at (fire and forget)
    let api_key_id = api_key.id;
    let db_clone = Arc::clone(db);
    tokio::spawn(async move {
        use sea_orm::ActiveModelTrait;
        use sea_orm::Set;

        if let Ok(Some(key)) = ApiKey::find_by_id(api_key_id).one(&*db_clone).await {
            let mut active_key: jility_core::api_key::ActiveModel = key.into();
            active_key.last_used_at = Set(Some(chrono::Utc::now()));
            let _ = active_key.update(&*db_clone).await;
        }
    });

    // Fetch user
    let user = User::find_by_id(api_key.user_id)
        .one(&**db)
        .await
        .map_err(|e| AppError::Database(e))?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Unauthorized(
            "User account is inactive".to_string(),
        ));
    }

    Ok(user.into())
}
