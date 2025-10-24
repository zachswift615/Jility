use axum::{extract::{Path, State}, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{LinkCommitRequest, CommitLinkResponse},
    state::AppState,
};
use jility_core::entities::{commit_link, CommitLink};

pub async fn link_commit(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<LinkCommitRequest>,
) -> ApiResult<Json<CommitLinkResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let commit_link = commit_link::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        commit_hash: Set(payload.commit_hash),
        commit_message: Set(payload.commit_message),
        linked_at: Set(Utc::now()),
        linked_by: Set("system".to_string()),
    };

    let result = commit_link
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(CommitLinkResponse {
        id: result.id.to_string(),
        commit_hash: result.commit_hash,
        commit_message: result.commit_message,
        linked_at: result.linked_at.to_rfc3339(),
        linked_by: result.linked_by,
    }))
}

pub async fn list_commits(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Vec<CommitLinkResponse>>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let commits = CommitLink::find()
        .filter(commit_link::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|c| CommitLinkResponse {
            id: c.id.to_string(),
            commit_hash: c.commit_hash,
            commit_message: c.commit_message,
            linked_at: c.linked_at.to_rfc3339(),
            linked_by: c.linked_by,
        })
        .collect();

    Ok(Json(commits))
}
