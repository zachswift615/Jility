use axum::{
    extract::{Path, Query, State},
    Extension,
    Json,
};
use chrono::Utc;
use jility_core::{
    entities::{saved_view, SavedView},
    search::{SearchFilters, SearchResponse as CoreSearchResponse},
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    error::{ApiError, ApiResult},
    models::{
        CreateSavedViewRequest, SavedViewResponse, SearchQuery, UpdateSavedViewRequest,
    },
    state::AppState,
};

/// Search tickets with full-text search and advanced filters
pub async fn search_tickets(
    State(state): State<AppState>,
    Extension(_auth_user): Extension<AuthUser>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<CoreSearchResponse>> {
    // Parse date filters
    let created_after = query
        .created_after
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let created_before = query
        .created_before
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let updated_after = query
        .updated_after
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let updated_before = query
        .updated_before
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    // Build search filters
    let filters = SearchFilters {
        query: query.q,
        status: if query.status.is_empty() {
            None
        } else {
            Some(query.status)
        },
        assignees: if query.assignees.is_empty() {
            None
        } else {
            Some(query.assignees)
        },
        labels: if query.labels.is_empty() {
            None
        } else {
            Some(query.labels)
        },
        created_by: query.created_by,
        created_after,
        created_before,
        updated_after,
        updated_before,
        min_points: query.min_points,
        max_points: query.max_points,
        has_comments: query.has_comments,
        has_commits: query.has_commits,
        has_dependencies: query.has_dependencies,
        epic_id: query.epic_id,
        parent_id: query.parent_id,
        project_id: query.project_id,
        search_in: if query.search_in.is_empty() {
            vec![
                "title".to_string(),
                "description".to_string(),
                "comments".to_string(),
            ]
        } else {
            query.search_in
        },
    };

    // Execute search
    let response = state
        .search_service
        .search_tickets(filters, Some(query.limit), Some(query.offset))
        .await
        .map_err(ApiError::from)?;

    Ok(Json(response))
}

/// Get all saved views for the current user
pub async fn list_saved_views(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<Vec<SavedViewResponse>>> {
    let views = SavedView::find()
        .filter(saved_view::Column::UserId.eq(&auth_user.id.to_string()))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let responses: Vec<SavedViewResponse> = views
        .into_iter()
        .map(|view| SavedViewResponse {
            id: view.id.to_string(),
            user_id: view.user_id.to_string(),
            name: view.name,
            description: view.description,
            filters: serde_json::from_str(&view.filters).unwrap_or(serde_json::json!({})),
            is_default: view.is_default,
            is_shared: view.is_shared,
            created_at: view.created_at.to_rfc3339(),
            updated_at: view.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(responses))
}

/// Get a specific saved view
pub async fn get_saved_view(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<SavedViewResponse>> {
    let view = SavedView::find_by_id(id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound("Saved view not found".to_string()))?;

    // Check if user owns this view or if it's shared
    if view.user_id.to_string() != auth_user.id.to_string() && !view.is_shared {
        return Err(ApiError::Forbidden(
            "You don't have permission to view this saved view".to_string(),
        ));
    }

    Ok(Json(SavedViewResponse {
        id: view.id.to_string(),
        user_id: view.user_id.to_string(),
        name: view.name,
        description: view.description,
        filters: serde_json::from_str(&view.filters).unwrap_or(serde_json::json!({})),
        is_default: view.is_default,
        is_shared: view.is_shared,
        created_at: view.created_at.to_rfc3339(),
        updated_at: view.updated_at.to_rfc3339(),
    }))
}

/// Create a new saved view
pub async fn create_saved_view(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateSavedViewRequest>,
) -> ApiResult<Json<SavedViewResponse>> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let user_id = Uuid::parse_str(&auth_user.id.to_string())
        .map_err(|_| ApiError::InvalidInput("Invalid user ID".to_string()))?;

    let view = saved_view::ActiveModel {
        id: ActiveValue::Set(id),
        user_id: ActiveValue::Set(user_id),
        name: ActiveValue::Set(req.name),
        description: ActiveValue::Set(req.description),
        filters: ActiveValue::Set(req.filters.to_string()),
        is_default: ActiveValue::Set(req.is_default.unwrap_or(false)),
        is_shared: ActiveValue::Set(req.is_shared.unwrap_or(false)),
        created_at: ActiveValue::Set(now.into()),
        updated_at: ActiveValue::Set(now.into()),
    };

    let view = view
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(SavedViewResponse {
        id: view.id.to_string(),
        user_id: view.user_id.to_string(),
        name: view.name,
        description: view.description,
        filters: serde_json::from_str(&view.filters).unwrap_or(serde_json::json!({})),
        is_default: view.is_default,
        is_shared: view.is_shared,
        created_at: view.created_at.to_rfc3339(),
        updated_at: view.updated_at.to_rfc3339(),
    }))
}

/// Update a saved view
pub async fn update_saved_view(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSavedViewRequest>,
) -> ApiResult<Json<SavedViewResponse>> {
    let view = SavedView::find_by_id(id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound("Saved view not found".to_string()))?;

    // Only owner can update
    if view.user_id.to_string() != auth_user.id.to_string() {
        return Err(ApiError::Forbidden(
            "You don't have permission to update this saved view".to_string(),
        ));
    }

    let mut active_view: saved_view::ActiveModel = view.into();

    if let Some(name) = req.name {
        active_view.name = ActiveValue::Set(name);
    }
    if let Some(description) = req.description {
        active_view.description = ActiveValue::Set(Some(description));
    }
    if let Some(filters) = req.filters {
        active_view.filters = ActiveValue::Set(filters.to_string());
    }
    if let Some(is_default) = req.is_default {
        active_view.is_default = ActiveValue::Set(is_default);
    }
    if let Some(is_shared) = req.is_shared {
        active_view.is_shared = ActiveValue::Set(is_shared);
    }

    active_view.updated_at = ActiveValue::Set(Utc::now().into());

    let view = active_view
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(SavedViewResponse {
        id: view.id.to_string(),
        user_id: view.user_id.to_string(),
        name: view.name,
        description: view.description,
        filters: serde_json::from_str(&view.filters).unwrap_or(serde_json::json!({})),
        is_default: view.is_default,
        is_shared: view.is_shared,
        created_at: view.created_at.to_rfc3339(),
        updated_at: view.updated_at.to_rfc3339(),
    }))
}

/// Delete a saved view
pub async fn delete_saved_view(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let view = SavedView::find_by_id(id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound("Saved view not found".to_string()))?;

    // Only owner can delete
    if view.user_id.to_string() != auth_user.id.to_string() {
        return Err(ApiError::Forbidden(
            "You don't have permission to delete this saved view".to_string(),
        ));
    }

    let active_view: saved_view::ActiveModel = view.into();
    active_view
        .delete(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({
        "message": "Saved view deleted successfully"
    })))
}
