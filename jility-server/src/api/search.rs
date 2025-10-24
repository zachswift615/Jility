use axum::{extract::{Query, State}, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    error::{ApiError, ApiResult},
    models::{SearchQuery, TicketResponse},
    state::AppState,
};
use jility_core::entities::{ticket, ticket_assignee, ticket_label, Ticket, TicketAssignee, TicketLabel};

pub async fn search_tickets(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<Vec<TicketResponse>>> {
    // Simple search - just search in title for now
    // TODO: Implement full-text search with FTS5 or similar
    let tickets = Ticket::find()
        .filter(ticket::Column::Title.contains(&query.q))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut responses = Vec::new();
    for ticket in tickets {
        let assignees = TicketAssignee::find()
            .filter(ticket_assignee::Column::TicketId.eq(ticket.id))
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .into_iter()
            .map(|a| a.assignee)
            .collect();

        let labels = TicketLabel::find()
            .filter(ticket_label::Column::TicketId.eq(ticket.id))
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .into_iter()
            .map(|l| l.label)
            .collect();

        responses.push(TicketResponse {
            id: ticket.id.to_string(),
            number: format!("TASK-{}", ticket.ticket_number),
            title: ticket.title,
            description: ticket.description,
            status: ticket.status,
            story_points: ticket.story_points,
            assignees,
            labels,
            created_at: ticket.created_at.to_rfc3339(),
            updated_at: ticket.updated_at.to_rfc3339(),
            created_by: ticket.created_by,
            parent_id: ticket.parent_id.map(|id| id.to_string()),
            epic_id: ticket.epic_id.map(|id| id.to_string()),
        });
    }

    Ok(Json(responses))
}
