use axum::{extract::{Path, State}, Extension, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    auth::middleware::AuthUser,
    error::{ApiError, ApiResult},
    models::{CreateCommentRequest, UpdateCommentRequest, CommentResponse},
    state::AppState,
};
use jility_core::entities::{comment, Comment};

pub async fn list_comments(
    State(state): State<AppState>,
    Path(ticket_id): Path<String>,
) -> ApiResult<Json<Vec<CommentResponse>>> {
    let ticket_uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", ticket_id)))?;

    let comments = Comment::find()
        .filter(comment::Column::TicketId.eq(ticket_uuid))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|c| CommentResponse {
            id: c.id.to_string(),
            ticket_id: c.ticket_id.to_string(),
            author: c.author,
            content: c.content,
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.map(|dt| dt.to_rfc3339()),
        })
        .collect();

    Ok(Json(comments))
}

pub async fn create_comment(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(ticket_id): Path<String>,
    Json(payload): Json<CreateCommentRequest>,
) -> ApiResult<Json<CommentResponse>> {
    let ticket_uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", ticket_id)))?;

    let now = Utc::now();
    let comment = comment::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_uuid),
        author: Set(auth_user.username),
        content: Set(payload.content),
        created_at: Set(now),
        updated_at: Set(None),
    };

    let result = comment
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let response = CommentResponse {
        id: result.id.to_string(),
        ticket_id: result.ticket_id.to_string(),
        author: result.author,
        content: result.content,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.map(|dt| dt.to_rfc3339()),
    };

    // Broadcast WebSocket update
    let ws_message = serde_json::to_string(&crate::models::ServerMessage::CommentAdded {
        ticket_id,
        comment: response.clone(),
    })
    .unwrap();
    state.ws_state.broadcast(ws_message).await;

    Ok(Json(response))
}

pub async fn update_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCommentRequest>,
) -> ApiResult<Json<CommentResponse>> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid comment ID: {}", id)))?;

    let comment = Comment::find_by_id(comment_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Comment not found: {}", id)))?;

    let mut comment: comment::ActiveModel = comment.into();
    let now = Utc::now();

    comment.content = Set(payload.content);
    comment.updated_at = Set(Some(now));

    let result = comment
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(CommentResponse {
        id: result.id.to_string(),
        ticket_id: result.ticket_id.to_string(),
        author: result.author,
        content: result.content,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.map(|dt| dt.to_rfc3339()),
    }))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid comment ID: {}", id)))?;

    Comment::delete_by_id(comment_id)
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({ "success": true })))
}
