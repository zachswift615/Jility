use axum::{extract::{Path, State}, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{AddDependencyRequest, DependencyGraphResponse, TicketReference},
    state::AppState,
};
use jility_core::entities::{ticket_dependency, Ticket, TicketDependency};

pub async fn add_dependency(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<AddDependencyRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let dependency = ticket_dependency::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        depends_on_id: Set(payload.depends_on_id),
        created_at: Set(Utc::now()),
        created_by: Set("system".to_string()),
    };

    dependency
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn remove_dependency(
    State(state): State<AppState>,
    Path((ticket_id, dep_id)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    let ticket_uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", ticket_id)))?;
    let dep_uuid = Uuid::parse_str(&dep_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid dependency ID: {}", dep_id)))?;

    // Find the dependency
    let dependency = TicketDependency::find()
        .filter(ticket_dependency::Column::TicketId.eq(ticket_uuid))
        .filter(ticket_dependency::Column::DependsOnId.eq(dep_uuid))
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound("Dependency not found".to_string()))?;

    TicketDependency::delete_by_id(dependency.id)
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn get_dependency_graph(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<DependencyGraphResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let ticket = Ticket::find_by_id(ticket_id)
        .filter(ticket::Column::DeletedAt.is_null())
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    // Get dependencies
    let dependencies = TicketDependency::find()
        .filter(ticket_dependency::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut dependency_refs = Vec::new();
    for dep in dependencies {
        if let Some(dep_ticket) = Ticket::find_by_id(dep.depends_on_id)
            .filter(ticket::Column::DeletedAt.is_null())
            .one(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
        {
            dependency_refs.push(TicketReference {
                id: dep_ticket.id.to_string(),
                number: format!("TASK-{}", dep_ticket.ticket_number),
                title: dep_ticket.title,
                status: dep_ticket.status,
            });
        }
    }

    // Get dependents
    let dependents = TicketDependency::find()
        .filter(ticket_dependency::Column::DependsOnId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut dependent_refs = Vec::new();
    for dep in dependents {
        if let Some(dep_ticket) = Ticket::find_by_id(dep.ticket_id)
            .filter(ticket::Column::DeletedAt.is_null())
            .one(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
        {
            dependent_refs.push(TicketReference {
                id: dep_ticket.id.to_string(),
                number: format!("TASK-{}", dep_ticket.ticket_number),
                title: dep_ticket.title,
                status: dep_ticket.status,
            });
        }
    }

    Ok(Json(DependencyGraphResponse {
        ticket: TicketReference {
            id: ticket.id.to_string(),
            number: format!("TASK-{}", ticket.ticket_number),
            title: ticket.title,
            status: ticket.status,
        },
        dependencies: dependency_refs,
        dependents: dependent_refs,
    }))
}
