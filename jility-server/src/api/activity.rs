use axum::{extract::{Path, State}, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    models::{ChangeEventResponse, HistoryVersionResponse},
    state::AppState,
};
use jility_core::entities::{ticket_change, TicketChange};

pub async fn get_activity(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Vec<ChangeEventResponse>>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .order_by_desc(ticket_change::Column::ChangedAt)
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|c| ChangeEventResponse {
            id: c.id.to_string(),
            change_type: c.change_type,
            field_name: c.field_name,
            old_value: c.old_value,
            new_value: c.new_value,
            changed_by: c.changed_by,
            changed_at: c.changed_at.to_rfc3339(),
            message: c.message,
        })
        .collect();

    Ok(Json(changes))
}

pub async fn get_history(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Vec<HistoryVersionResponse>>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .filter(ticket_change::Column::ChangeType.eq("description_changed"))
        .order_by_desc(ticket_change::Column::ChangedAt)
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let versions = changes
        .into_iter()
        .enumerate()
        .map(|(idx, c)| HistoryVersionResponse {
            version: (idx + 1) as i32,
            description: c.new_value.unwrap_or_default(),
            changed_by: c.changed_by,
            changed_at: c.changed_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(versions))
}

pub async fn get_version(
    State(state): State<AppState>,
    Path((id, version)): Path<(String, i32)>,
) -> ApiResult<Json<HistoryVersionResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .filter(ticket_change::Column::ChangeType.eq("description_changed"))
        .order_by_desc(ticket_change::Column::ChangedAt)
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let change = changes
        .get((version - 1) as usize)
        .ok_or_else(|| ApiError::NotFound(format!("Version {} not found", version)))?;

    Ok(Json(HistoryVersionResponse {
        version,
        description: change.new_value.clone().unwrap_or_default(),
        changed_by: change.changed_by.clone(),
        changed_at: change.changed_at.to_rfc3339(),
    }))
}

pub async fn revert_to_version(
    State(_state): State<AppState>,
    Path((_id, _version)): Path<(String, i32)>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: Implement revert functionality
    Ok(Json(serde_json::json!({ "success": true, "message": "Not implemented yet" })))
}
