use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use jility_core::{ApiKey, Session, User};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

// ===== Request/Response Types =====

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: i64,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
}

impl From<jility_core::UserModel> for UserResponse {
    fn from(user: jility_core::UserModel) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            username: user.username,
            full_name: user.full_name,
            avatar_url: user.avatar_url,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_in_days: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyCreatedResponse {
    pub api_key: ApiKeyResponse,
    pub key: String, // Only returned on creation
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
    pub expires_at: String,
}

// ===== Handlers =====

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    // Validate input
    if req.email.is_empty() || !req.email.contains('@') {
        return Err(ApiError::Validation("Invalid email address".to_string()));
    }

    if req.username.is_empty() || req.username.len() < 3 {
        return Err(ApiError::Validation(
            "Username must be at least 3 characters".to_string(),
        ));
    }

    if req.password.len() < 8 {
        return Err(ApiError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Check if at least one number in password
    if !req.password.chars().any(|c| c.is_numeric()) {
        return Err(ApiError::Validation(
            "Password must contain at least one number".to_string(),
        ));
    }

    // Check if user already exists
    let existing_user = User::find()
        .filter(jility_core::user::Column::Email.eq(&req.email))
        .one(&*state.db)
        .await?;

    if existing_user.is_some() {
        return Err(ApiError::Validation(
            "Email already registered".to_string(),
        ));
    }

    let existing_username = User::find()
        .filter(jility_core::user::Column::Username.eq(&req.username))
        .one(&*state.db)
        .await?;

    if existing_username.is_some() {
        return Err(ApiError::Validation("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = state
        .auth_service
        .hash_password(&req.password)
        .map_err(|e| ApiError::Internal(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let user = jility_core::user::ActiveModel {
        id: Set(user_id),
        email: Set(req.email.clone()),
        username: Set(req.username.clone()),
        password_hash: Set(password_hash),
        full_name: Set(req.full_name.clone()),
        avatar_url: Set(None),
        is_active: Set(true),
        is_verified: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        last_login: Set(Some(now)),
    };

    let user = user.insert(&*state.db).await?;

    // Generate JWT
    let token = state
        .auth_service
        .generate_jwt(user_id)
        .map_err(|e| ApiError::Internal(format!("Failed to generate token: {}", e)))?;

    // Create session
    let expires_at = now + chrono::Duration::days(7);
    let token_hash = state
        .auth_service
        .hash_token(&token)
        .map_err(|e| ApiError::Internal(format!("Failed to hash token: {}", e)))?;

    let session = jility_core::session::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        token_hash: Set(token_hash),
        ip_address: Set(None),
        user_agent: Set(None),
        created_at: Set(now),
        expires_at: Set(expires_at),
        revoked_at: Set(None),
    };

    session.insert(&*state.db).await?;

    Ok(Json(AuthResponse {
        token,
        expires_at: expires_at.timestamp(),
        user: user.into(),
    }))
}

/// Login user
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    // Find user by email
    let user = User::find()
        .filter(jility_core::user::Column::Email.eq(&req.email))
        .one(&*state.db)
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Invalid email or password".to_string()))?;

    // Verify password
    let password_valid = state
        .auth_service
        .verify_password(&req.password, &user.password_hash)
        .map_err(|e| ApiError::Internal(format!("Failed to verify password: {}", e)))?;

    if !password_valid {
        return Err(ApiError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // Check if user is active
    if !user.is_active {
        return Err(ApiError::Unauthorized(
            "User account is inactive".to_string(),
        ));
    }

    // Update last_login
    let mut active_user: jility_core::user::ActiveModel = user.clone().into();
    active_user.last_login = Set(Some(chrono::Utc::now()));
    active_user.update(&*state.db).await?;

    // Generate JWT
    let token = state
        .auth_service
        .generate_jwt(user.id)
        .map_err(|e| ApiError::Internal(format!("Failed to generate token: {}", e)))?;

    // Create session
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::days(7);
    let token_hash = state
        .auth_service
        .hash_token(&token)
        .map_err(|e| ApiError::Internal(format!("Failed to hash token: {}", e)))?;

    let session = jility_core::session::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        token_hash: Set(token_hash),
        ip_address: Set(None),
        user_agent: Set(None),
        created_at: Set(now),
        expires_at: Set(expires_at),
        revoked_at: Set(None),
    };

    session.insert(&*state.db).await?;

    Ok(Json(AuthResponse {
        token,
        expires_at: expires_at.timestamp(),
        user: user.into(),
    }))
}

/// Logout user (revoke current session)
pub async fn logout(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    // Get token from request (this would need to be passed through middleware)
    // For now, we'll revoke all sessions for the user
    // In production, you'd want to track the specific session

    Session::update_many()
        .col_expr(
            jility_core::session::Column::RevokedAt,
            sea_orm::sea_query::Expr::value(chrono::Utc::now()),
        )
        .filter(jility_core::session::Column::UserId.eq(auth_user.id))
        .filter(jility_core::session::Column::RevokedAt.is_null())
        .exec(&*state.db)
        .await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Get current user
pub async fn get_me(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<UserResponse>> {
    let user = User::find_by_id(auth_user.id)
        .one(&*state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}

/// Create API key
pub async fn create_api_key(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateApiKeyRequest>,
) -> ApiResult<Json<ApiKeyCreatedResponse>> {
    // Validate input
    if req.name.is_empty() {
        return Err(ApiError::Validation("API key name is required".to_string()));
    }

    // Generate API key
    let (key, key_hash) = state
        .auth_service
        .generate_api_key()
        .map_err(|e| ApiError::Internal(format!("Failed to generate API key: {}", e)))?;

    let prefix = crate::auth::service::AuthService::get_key_prefix(&key)
        .map_err(|e| ApiError::Internal(format!("Failed to get key prefix: {}", e)))?;

    // Create API key record
    let now = chrono::Utc::now();
    let expires_at = req.expires_in_days.map(|days| now + chrono::Duration::days(days));

    let api_key = jility_core::api_key::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(auth_user.id),
        name: Set(req.name.clone()),
        key_hash: Set(key_hash),
        prefix: Set(prefix.clone()),
        scopes: Set(serde_json::to_string(&req.scopes).unwrap_or_else(|_| "[]".to_string())),
        expires_at: Set(expires_at),
        last_used_at: Set(None),
        created_at: Set(now),
        revoked_at: Set(None),
    };

    let api_key = api_key.insert(&*state.db).await?;

    Ok(Json(ApiKeyCreatedResponse {
        api_key: ApiKeyResponse {
            id: api_key.id.to_string(),
            name: api_key.name,
            prefix: api_key.prefix,
            scopes: serde_json::from_str(&api_key.scopes).unwrap_or_default(),
            created_at: api_key.created_at.to_rfc3339(),
            expires_at: api_key.expires_at.map(|dt| dt.to_rfc3339()),
            last_used_at: api_key.last_used_at.map(|dt| dt.to_rfc3339()),
        },
        key, // Only returned on creation
    }))
}

/// List API keys for current user
pub async fn list_api_keys(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<Vec<ApiKeyResponse>>> {
    let api_keys = ApiKey::find()
        .filter(jility_core::api_key::Column::UserId.eq(auth_user.id))
        .filter(jility_core::api_key::Column::RevokedAt.is_null())
        .all(&*state.db)
        .await?;

    let response: Vec<ApiKeyResponse> = api_keys
        .into_iter()
        .map(|key| ApiKeyResponse {
            id: key.id.to_string(),
            name: key.name,
            prefix: key.prefix,
            scopes: serde_json::from_str(&key.scopes).unwrap_or_default(),
            created_at: key.created_at.to_rfc3339(),
            expires_at: key.expires_at.map(|dt| dt.to_rfc3339()),
            last_used_at: key.last_used_at.map(|dt| dt.to_rfc3339()),
        })
        .collect();

    Ok(Json(response))
}

/// Revoke API key
pub async fn revoke_api_key(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(key_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|_| ApiError::InvalidInput("Invalid API key ID".to_string()))?;

    // Find API key
    let api_key = ApiKey::find_by_id(key_uuid)
        .one(&*state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;

    // Verify ownership
    if api_key.user_id != auth_user.id {
        return Err(ApiError::Unauthorized(
            "You don't have permission to revoke this API key".to_string(),
        ));
    }

    // Revoke key
    let mut active_key: jility_core::api_key::ActiveModel = api_key.into();
    active_key.revoked_at = Set(Some(chrono::Utc::now()));
    active_key.update(&*state.db).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// List sessions for current user
pub async fn list_sessions(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<Vec<SessionResponse>>> {
    let sessions = Session::find()
        .filter(jility_core::session::Column::UserId.eq(auth_user.id))
        .filter(jility_core::session::Column::RevokedAt.is_null())
        .all(&*state.db)
        .await?;

    let response: Vec<SessionResponse> = sessions
        .into_iter()
        .map(|session| SessionResponse {
            id: session.id.to_string(),
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            created_at: session.created_at.to_rfc3339(),
            expires_at: session.expires_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}
