use axum::{extract::{Path, State}, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    state::AppState,
};
use jility_core::entities::{
    ticket, Ticket,
};

/// Response for epic with progress information
#[derive(Debug, Serialize)]
pub struct EpicResponse {
    pub id: String,
    pub number: String,
    pub title: String,
    pub description: String,
    pub epic_color: Option<String>,
    pub progress: EpicProgress,
    pub created_at: String,
    pub updated_at: String,
}

/// Epic progress statistics
#[derive(Debug, Serialize)]
pub struct EpicProgress {
    pub total: i32,
    pub done: i32,
    pub in_progress: i32,
    pub todo: i32,
    pub blocked: i32,
    pub completion_percentage: i32,
}

/// Helper function to format ticket number with project key
async fn format_ticket_number(
    db: &sea_orm::DatabaseConnection,
    ticket: &ticket::Model,
) -> ApiResult<String> {
    use jility_core::entities::{project, Project};

    let project = Project::find_by_id(ticket.project_id)
        .one(db)
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Project not found: {}", ticket.project_id)))?;

    let prefix = project.key.as_ref().map(|k| k.as_str()).unwrap_or("TASK");
    Ok(format!("{}-{}", prefix, ticket.ticket_number))
}

/// Calculate epic progress from child tickets
async fn calculate_epic_progress(
    db: &sea_orm::DatabaseConnection,
    epic_id: Uuid,
) -> ApiResult<EpicProgress> {
    let tickets = Ticket::find()
        .filter(ticket::Column::EpicId.eq(epic_id))
        .filter(ticket::Column::DeletedAt.is_null())
        .all(db)
        .await
        .map_err(ApiError::from)?;

    let total = tickets.len() as i32;
    let mut done = 0;
    let mut in_progress = 0;
    let mut todo = 0;
    let mut blocked = 0;

    for ticket in tickets {
        match ticket.status.as_str() {
            "done" => done += 1,
            "in_progress" | "review" => in_progress += 1,
            "todo" | "backlog" => todo += 1,
            "blocked" => blocked += 1,
            _ => todo += 1,
        }
    }

    let completion_percentage = if total > 0 {
        (done * 100) / total
    } else {
        0
    };

    Ok(EpicProgress {
        total,
        done,
        in_progress,
        todo,
        blocked,
        completion_percentage,
    })
}

/// List all epics with progress stats
pub async fn list_epics(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<EpicResponse>>> {
    let epics = Ticket::find()
        .filter(ticket::Column::IsEpic.eq(true))
        .filter(ticket::Column::DeletedAt.is_null())
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut responses = Vec::new();

    for epic in epics {
        let number = format_ticket_number(state.db.as_ref(), &epic).await?;
        let progress = calculate_epic_progress(state.db.as_ref(), epic.id).await?;

        responses.push(EpicResponse {
            id: epic.id.to_string(),
            number,
            title: epic.title,
            description: epic.description,
            epic_color: epic.epic_color,
            progress,
            created_at: epic.created_at.to_rfc3339(),
            updated_at: epic.updated_at.to_rfc3339(),
        });
    }

    Ok(Json(responses))
}

/// Get a specific epic with progress
pub async fn get_epic(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<EpicResponse>> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid epic ID: {}", id)))?;

    let epic = Ticket::find_by_id(uuid)
        .filter(ticket::Column::IsEpic.eq(true))
        .filter(ticket::Column::DeletedAt.is_null())
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Epic not found: {}", id)))?;

    let number = format_ticket_number(state.db.as_ref(), &epic).await?;
    let progress = calculate_epic_progress(state.db.as_ref(), epic.id).await?;

    Ok(Json(EpicResponse {
        id: epic.id.to_string(),
        number,
        title: epic.title,
        description: epic.description,
        epic_color: epic.epic_color,
        progress,
        created_at: epic.created_at.to_rfc3339(),
        updated_at: epic.updated_at.to_rfc3339(),
    }))
}

/// Get all tickets for a specific epic
pub async fn get_epic_tickets(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Vec<crate::models::TicketResponse>>> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid epic ID: {}", id)))?;

    // Verify epic exists
    let epic = Ticket::find_by_id(uuid)
        .filter(ticket::Column::IsEpic.eq(true))
        .filter(ticket::Column::DeletedAt.is_null())
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Epic not found: {}", id)))?;

    // Get all tickets belonging to this epic
    let tickets = Ticket::find()
        .filter(ticket::Column::EpicId.eq(epic.id))
        .filter(ticket::Column::DeletedAt.is_null())
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut responses = Vec::new();

    for ticket in tickets {
        use jility_core::entities::{ticket_assignee, ticket_label, TicketAssignee, TicketLabel};

        // Get assignees
        let assignees = TicketAssignee::find()
            .filter(ticket_assignee::Column::TicketId.eq(ticket.id))
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .into_iter()
            .map(|a| a.assignee)
            .collect();

        // Get labels
        let labels = TicketLabel::find()
            .filter(ticket_label::Column::TicketId.eq(ticket.id))
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .into_iter()
            .map(|l| l.label)
            .collect();

        let number = format_ticket_number(state.db.as_ref(), &ticket).await?;

        responses.push(crate::models::TicketResponse {
            id: ticket.id.to_string(),
            number,
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
            is_epic: ticket.is_epic,
            epic_color: ticket.epic_color,
        });
    }

    Ok(Json(responses))
}
