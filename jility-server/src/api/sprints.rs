use axum::{extract::{Path, Query, State}, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{
        CreateSprintRequest, UpdateSprintRequest, StartSprintRequest, AddTicketToSprintRequest,
        SprintResponse, SprintDetailsResponse, SprintStats, BurndownData, BurndownDataPoint,
        SprintHistoryResponse, VelocityData, TicketResponse, format_uuid, format_datetime,
    },
    state::AppState,
};
use jility_core::entities::{
    sprint, sprint_ticket, ticket, ticket_change,
    Sprint, SprintTicket, Ticket, TicketChange, ChangeType,
};

#[derive(Debug, Deserialize)]
pub struct ListSprintsQuery {
    pub status: Option<String>,
}

/// List all sprints for a project
pub async fn list_sprints(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Query(query): Query<ListSprintsQuery>,
) -> ApiResult<Json<Vec<SprintResponse>>> {
    let project_uuid = Uuid::parse_str(&project_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", project_id)))?;

    let mut query_builder = Sprint::find()
        .filter(sprint::Column::ProjectId.eq(project_uuid));

    if let Some(status) = query.status {
        query_builder = query_builder.filter(sprint::Column::Status.eq(status));
    }

    let sprints = query_builder
        .order_by_desc(sprint::Column::CreatedAt)
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let responses: Vec<SprintResponse> = sprints
        .into_iter()
        .map(|s| SprintResponse {
            id: format_uuid(&s.id),
            project_id: format_uuid(&s.project_id),
            name: s.name,
            goal: s.goal,
            status: s.status,
            start_date: s.start_date.map(|d| format_datetime(&d)),
            end_date: s.end_date.map(|d| format_datetime(&d)),
            created_at: format_datetime(&s.created_at),
            updated_at: format_datetime(&s.updated_at),
        })
        .collect();

    Ok(Json(responses))
}

/// Create a new sprint
pub async fn create_sprint(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateSprintRequest>,
) -> ApiResult<Json<SprintResponse>> {
    let project_uuid = Uuid::parse_str(&project_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", project_id)))?;

    let start_date = if let Some(date_str) = req.start_date {
        Some(chrono::DateTime::parse_from_rfc3339(&date_str)
            .map_err(|_| ApiError::InvalidInput("Invalid start_date format".to_string()))?
            .with_timezone(&Utc))
    } else {
        None
    };

    let end_date = if let Some(date_str) = req.end_date {
        Some(chrono::DateTime::parse_from_rfc3339(&date_str)
            .map_err(|_| ApiError::InvalidInput("Invalid end_date format".to_string()))?
            .with_timezone(&Utc))
    } else {
        None
    };

    let now = Utc::now();
    let sprint_id = Uuid::new_v4();

    let sprint = sprint::ActiveModel {
        id: Set(sprint_id),
        project_id: Set(project_uuid),
        name: Set(req.name.clone()),
        goal: Set(req.goal.clone()),
        start_date: Set(start_date),
        end_date: Set(end_date),
        status: Set("planning".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let sprint = sprint.insert(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(SprintResponse {
        id: format_uuid(&sprint.id),
        project_id: format_uuid(&sprint.project_id),
        name: sprint.name,
        goal: sprint.goal,
        status: sprint.status,
        start_date: sprint.start_date.map(|d| format_datetime(&d)),
        end_date: sprint.end_date.map(|d| format_datetime(&d)),
        created_at: format_datetime(&sprint.created_at),
        updated_at: format_datetime(&sprint.updated_at),
    }))
}

/// Get sprint details with tickets and stats
pub async fn get_sprint(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
) -> ApiResult<Json<SprintDetailsResponse>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    let sprint = Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    // Get all tickets in this sprint
    let sprint_tickets = SprintTicket::find()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_uuid))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let ticket_ids: Vec<Uuid> = sprint_tickets.iter().map(|st| st.ticket_id).collect();

    let tickets = if ticket_ids.is_empty() {
        vec![]
    } else {
        Ticket::find()
            .filter(ticket::Column::Id.is_in(ticket_ids))
            .filter(ticket::Column::DeletedAt.is_null())
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
    };

    // Calculate stats
    let total_tickets = tickets.len();
    let total_points: i32 = tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let completed_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.status == "done")
        .collect();

    let in_progress_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.status == "in_progress")
        .collect();

    let todo_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.status == "todo" || t.status == "backlog")
        .collect();

    let completed_points: i32 = completed_tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let in_progress_points: i32 = in_progress_tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let todo_points: i32 = todo_tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let completion_percentage = if total_points > 0 {
        (completed_points as f64 / total_points as f64) * 100.0
    } else {
        0.0
    };

    let stats = SprintStats {
        total_tickets,
        total_points,
        completed_tickets: completed_tickets.len(),
        completed_points,
        in_progress_tickets: in_progress_tickets.len(),
        in_progress_points,
        todo_tickets: todo_tickets.len(),
        todo_points,
        completion_percentage,
    };

    let ticket_responses: Vec<TicketResponse> = tickets
        .into_iter()
        .map(|t| TicketResponse {
            id: format_uuid(&t.id),
            number: format!("TASK-{}", t.ticket_number),
            title: t.title.clone(),
            description: t.description.clone(),
            status: t.status.clone(),
            story_points: t.story_points,
            assignees: vec![],
            labels: vec![],
            created_at: format_datetime(&t.created_at),
            updated_at: format_datetime(&t.updated_at),
            created_by: t.created_by.clone(),
            parent_id: t.parent_id.map(|id| format_uuid(&id)),
            epic_id: t.epic_id.map(|id| format_uuid(&id)),
            is_epic: t.is_epic,
            epic_color: t.epic_color.clone(),
        })
        .collect();

    Ok(Json(SprintDetailsResponse {
        sprint: SprintResponse {
            id: format_uuid(&sprint.id),
            project_id: format_uuid(&sprint.project_id),
            name: sprint.name,
            goal: sprint.goal,
            status: sprint.status,
            start_date: sprint.start_date.map(|d| format_datetime(&d)),
            end_date: sprint.end_date.map(|d| format_datetime(&d)),
            created_at: format_datetime(&sprint.created_at),
            updated_at: format_datetime(&sprint.updated_at),
        },
        tickets: ticket_responses,
        stats,
    }))
}

/// Update sprint
pub async fn update_sprint(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
    Json(req): Json<UpdateSprintRequest>,
) -> ApiResult<Json<SprintResponse>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    let sprint = Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    let mut active_sprint: sprint::ActiveModel = sprint.into();

    if let Some(name) = req.name {
        active_sprint.name = Set(name);
    }

    if let Some(goal) = req.goal {
        active_sprint.goal = Set(Some(goal));
    }

    if let Some(start_date_str) = req.start_date {
        let start_date = chrono::DateTime::parse_from_rfc3339(&start_date_str)
            .map_err(|_| ApiError::InvalidInput("Invalid start_date format".to_string()))?
            .with_timezone(&Utc);
        active_sprint.start_date = Set(Some(start_date));
    }

    if let Some(end_date_str) = req.end_date {
        let end_date = chrono::DateTime::parse_from_rfc3339(&end_date_str)
            .map_err(|_| ApiError::InvalidInput("Invalid end_date format".to_string()))?
            .with_timezone(&Utc);
        active_sprint.end_date = Set(Some(end_date));
    }

    active_sprint.updated_at = Set(Utc::now());

    let sprint = active_sprint.update(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(SprintResponse {
        id: format_uuid(&sprint.id),
        project_id: format_uuid(&sprint.project_id),
        name: sprint.name,
        goal: sprint.goal,
        status: sprint.status,
        start_date: sprint.start_date.map(|d| format_datetime(&d)),
        end_date: sprint.end_date.map(|d| format_datetime(&d)),
        created_at: format_datetime(&sprint.created_at),
        updated_at: format_datetime(&sprint.updated_at),
    }))
}

/// Delete sprint
pub async fn delete_sprint(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
) -> ApiResult<Json<()>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    let sprint = Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    // Delete sprint_tickets entries first
    SprintTicket::delete_many()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_uuid))
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Delete sprint
    let active_sprint: sprint::ActiveModel = sprint.into();
    active_sprint.delete(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(()))
}

/// Start a sprint
pub async fn start_sprint(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
    Json(req): Json<StartSprintRequest>,
) -> ApiResult<Json<SprintResponse>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    let sprint = Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    if sprint.status != "planning" {
        return Err(ApiError::InvalidInput("Sprint has already been started".to_string()));
    }

    let start_date = chrono::DateTime::parse_from_rfc3339(&req.start_date)
        .map_err(|_| ApiError::InvalidInput("Invalid start_date format".to_string()))?
        .with_timezone(&Utc);

    let end_date = chrono::DateTime::parse_from_rfc3339(&req.end_date)
        .map_err(|_| ApiError::InvalidInput("Invalid end_date format".to_string()))?
        .with_timezone(&Utc);

    let mut active_sprint: sprint::ActiveModel = sprint.into();
    active_sprint.status = Set("active".to_string());
    active_sprint.start_date = Set(Some(start_date));
    active_sprint.end_date = Set(Some(end_date));
    active_sprint.updated_at = Set(Utc::now());

    let sprint = active_sprint.update(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(SprintResponse {
        id: format_uuid(&sprint.id),
        project_id: format_uuid(&sprint.project_id),
        name: sprint.name,
        goal: sprint.goal,
        status: sprint.status,
        start_date: sprint.start_date.map(|d| format_datetime(&d)),
        end_date: sprint.end_date.map(|d| format_datetime(&d)),
        created_at: format_datetime(&sprint.created_at),
        updated_at: format_datetime(&sprint.updated_at),
    }))
}

/// Complete a sprint
pub async fn complete_sprint(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
) -> ApiResult<Json<SprintResponse>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    let sprint = Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    if sprint.status != "active" {
        return Err(ApiError::InvalidInput("Only active sprints can be completed".to_string()));
    }

    let mut active_sprint: sprint::ActiveModel = sprint.into();
    active_sprint.status = Set("completed".to_string());
    active_sprint.updated_at = Set(Utc::now());

    let sprint = active_sprint.update(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(SprintResponse {
        id: format_uuid(&sprint.id),
        project_id: format_uuid(&sprint.project_id),
        name: sprint.name,
        goal: sprint.goal,
        status: sprint.status,
        start_date: sprint.start_date.map(|d| format_datetime(&d)),
        end_date: sprint.end_date.map(|d| format_datetime(&d)),
        created_at: format_datetime(&sprint.created_at),
        updated_at: format_datetime(&sprint.updated_at),
    }))
}

/// Add a ticket to a sprint
pub async fn add_ticket_to_sprint(
    State(state): State<AppState>,
    Path((sprint_id, ticket_id)): Path<(String, String)>,
    Json(req): Json<AddTicketToSprintRequest>,
) -> ApiResult<Json<()>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;
    let ticket_uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", ticket_id)))?;

    // Verify sprint exists
    Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    // Verify ticket exists
    Ticket::find_by_id(ticket_uuid)
        .filter(ticket::Column::DeletedAt.is_null())
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket {} not found", ticket_id)))?;

    // Check if ticket is already in sprint
    let existing = SprintTicket::find()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_uuid))
        .filter(sprint_ticket::Column::TicketId.eq(ticket_uuid))
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    if existing.is_some() {
        return Err(ApiError::InvalidInput("Ticket is already in this sprint".to_string()));
    }

    let now = Utc::now();
    let sprint_ticket = sprint_ticket::ActiveModel {
        id: Set(Uuid::new_v4()),
        sprint_id: Set(sprint_uuid),
        ticket_id: Set(ticket_uuid),
        added_at: Set(now),
        added_by: Set(req.added_by.clone()),
    };

    sprint_ticket.insert(state.db.as_ref()).await.map_err(ApiError::from)?;

    // Record change event
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_uuid),
        change_type: Set(ChangeType::AddedToSprint.as_str().to_string()),
        field_name: Set(None),
        old_value: Set(None),
        new_value: Set(Some(format_uuid(&sprint_uuid))),
        changed_by: Set(req.added_by),
        changed_at: Set(now),
        message: Set(None),
    };

    change.insert(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(()))
}

/// Remove a ticket from a sprint
pub async fn remove_ticket_from_sprint(
    State(state): State<AppState>,
    Path((sprint_id, ticket_id)): Path<(String, String)>,
) -> ApiResult<Json<()>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;
    let ticket_uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", ticket_id)))?;

    // Delete sprint_ticket entry
    let result = SprintTicket::delete_many()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_uuid))
        .filter(sprint_ticket::Column::TicketId.eq(ticket_uuid))
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    if result.rows_affected == 0 {
        return Err(ApiError::NotFound("Ticket not found in sprint".to_string()));
    }

    // Record change event
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_uuid),
        change_type: Set(ChangeType::RemovedFromSprint.as_str().to_string()),
        field_name: Set(None),
        old_value: Set(Some(format_uuid(&sprint_uuid))),
        new_value: Set(None),
        changed_by: Set("system".to_string()),
        changed_at: Set(Utc::now()),
        message: Set(None),
    };

    change.insert(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(()))
}

/// Get sprint statistics
pub async fn get_sprint_stats(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
) -> ApiResult<Json<SprintStats>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    // Get all tickets in this sprint
    let sprint_tickets = SprintTicket::find()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_uuid))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let ticket_ids: Vec<Uuid> = sprint_tickets.iter().map(|st| st.ticket_id).collect();

    let tickets = if ticket_ids.is_empty() {
        vec![]
    } else {
        Ticket::find()
            .filter(ticket::Column::Id.is_in(ticket_ids))
            .filter(ticket::Column::DeletedAt.is_null())
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
    };

    let total_tickets = tickets.len();
    let total_points: i32 = tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let completed_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.status == "done")
        .collect();

    let in_progress_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.status == "in_progress")
        .collect();

    let todo_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.status == "todo" || t.status == "backlog")
        .collect();

    let completed_points: i32 = completed_tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let in_progress_points: i32 = in_progress_tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let todo_points: i32 = todo_tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    let completion_percentage = if total_points > 0 {
        (completed_points as f64 / total_points as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(SprintStats {
        total_tickets,
        total_points,
        completed_tickets: completed_tickets.len(),
        completed_points,
        in_progress_tickets: in_progress_tickets.len(),
        in_progress_points,
        todo_tickets: todo_tickets.len(),
        todo_points,
        completion_percentage,
    }))
}

/// Get burndown chart data
pub async fn get_burndown(
    State(state): State<AppState>,
    Path(sprint_id): Path<String>,
) -> ApiResult<Json<BurndownData>> {
    let sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", sprint_id)))?;

    let sprint = Sprint::find_by_id(sprint_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint {} not found", sprint_id)))?;

    let start_date = sprint.start_date.ok_or_else(||
        ApiError::InvalidInput("Sprint has no start date".to_string()))?;
    let end_date = sprint.end_date.ok_or_else(||
        ApiError::InvalidInput("Sprint has no end date".to_string()))?;

    // Get all tickets in sprint
    let sprint_tickets = SprintTicket::find()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_uuid))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let ticket_ids: Vec<Uuid> = sprint_tickets.iter().map(|st| st.ticket_id).collect();

    let tickets = if ticket_ids.is_empty() {
        vec![]
    } else {
        Ticket::find()
            .filter(ticket::Column::Id.is_in(ticket_ids))
            .filter(ticket::Column::DeletedAt.is_null())
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
    };

    let total_points: i32 = tickets
        .iter()
        .filter_map(|t| t.story_points)
        .sum();

    // Calculate number of days
    let days = (end_date - start_date).num_days() + 1;

    // Generate data points
    let mut data_points = Vec::new();

    for day in 0..days {
        let current_date = start_date + chrono::Duration::days(day);
        let date_str = current_date.format("%Y-%m-%d").to_string();

        // Ideal burndown (linear)
        let ideal = if days > 1 {
            total_points - ((total_points * day as i32) / (days - 1) as i32)
        } else {
            total_points
        };

        // Actual remaining points - count tickets that were not "done" by this date
        // This is simplified - in reality you'd query ticket_changes to see status at that time
        let actual = if current_date > Utc::now() {
            // Future dates - use ideal as estimate
            ideal
        } else {
            // For simplicity, calculate based on current status
            // A real implementation would query ticket_changes table
            let completed_by_date: i32 = tickets
                .iter()
                .filter(|t| {
                    t.status == "done" && t.updated_at <= current_date
                })
                .filter_map(|t| t.story_points)
                .sum();

            total_points - completed_by_date
        };

        data_points.push(BurndownDataPoint {
            date: date_str,
            ideal,
            actual,
        });
    }

    Ok(Json(BurndownData {
        sprint_id: format_uuid(&sprint_uuid),
        data_points,
    }))
}

/// Get sprint history with velocity data
pub async fn get_sprint_history(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> ApiResult<Json<SprintHistoryResponse>> {
    let project_uuid = Uuid::parse_str(&project_id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", project_id)))?;

    let sprints = Sprint::find()
        .filter(sprint::Column::ProjectId.eq(project_uuid))
        .filter(sprint::Column::Status.eq("completed"))
        .order_by_desc(sprint::Column::CreatedAt)
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut velocity_data = Vec::new();
    let mut total_velocity = 0;

    for sprint in &sprints {
        // Get completed points for this sprint
        let sprint_tickets = SprintTicket::find()
            .filter(sprint_ticket::Column::SprintId.eq(sprint.id))
            .all(state.db.as_ref())
            .await
            .map_err(ApiError::from)?;

        let ticket_ids: Vec<Uuid> = sprint_tickets.iter().map(|st| st.ticket_id).collect();

        let completed_points: i32 = if ticket_ids.is_empty() {
            0
        } else {
            let tickets = Ticket::find()
                .filter(ticket::Column::Id.is_in(ticket_ids))
                .filter(ticket::Column::Status.eq("done"))
                .all(state.db.as_ref())
                .await
                .map_err(ApiError::from)?;

            tickets.iter().filter_map(|t| t.story_points).sum()
        };

        total_velocity += completed_points;

        velocity_data.push(VelocityData {
            sprint_name: sprint.name.clone(),
            completed_points,
        });
    }

    let average_velocity = if !sprints.is_empty() {
        total_velocity as f64 / sprints.len() as f64
    } else {
        0.0
    };

    let sprint_responses: Vec<SprintResponse> = sprints
        .into_iter()
        .map(|s| SprintResponse {
            id: format_uuid(&s.id),
            project_id: format_uuid(&s.project_id),
            name: s.name,
            goal: s.goal,
            status: s.status,
            start_date: s.start_date.map(|d| format_datetime(&d)),
            end_date: s.end_date.map(|d| format_datetime(&d)),
            created_at: format_datetime(&s.created_at),
            updated_at: format_datetime(&s.updated_at),
        })
        .collect();

    Ok(Json(SprintHistoryResponse {
        sprints: sprint_responses,
        velocity_data,
        average_velocity,
    }))
}
