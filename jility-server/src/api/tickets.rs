use axum::{extract::{Path, Query, State}, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{
        CreateTicketRequest, UpdateTicketRequest, UpdateDescriptionRequest, UpdateStatusRequest,
        AssignTicketRequest, UnassignTicketRequest, TicketResponse, TicketDetailResponse,
        CommentResponse, TicketReference, CommitLinkResponse, ChangeEventResponse,
    },
    state::AppState,
};
use jility_core::entities::{
    ticket, ticket_assignee, ticket_label, ticket_change, comment, commit_link,
    Ticket, TicketAssignee, TicketLabel, TicketChange, Comment, CommitLink,
    TicketDependency, TicketStatus, ChangeType,
};

#[derive(Debug, Deserialize)]
pub struct ListTicketsQuery {
    pub project_id: Option<String>,
    pub status: Option<String>,
    pub assignee: Option<String>,
}

pub async fn list_tickets(
    State(state): State<AppState>,
    Query(query): Query<ListTicketsQuery>,
) -> ApiResult<Json<Vec<TicketResponse>>> {
    let mut query_builder = Ticket::find();

    if let Some(project_id) = query.project_id {
        let uuid = Uuid::parse_str(&project_id)
            .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", project_id)))?;
        query_builder = query_builder.filter(ticket::Column::ProjectId.eq(uuid));
    }

    if let Some(status) = query.status {
        query_builder = query_builder.filter(ticket::Column::Status.eq(status));
    }

    let tickets = query_builder
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut responses = Vec::new();

    for ticket in tickets {
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

pub async fn create_ticket(
    State(state): State<AppState>,
    Json(payload): Json<CreateTicketRequest>,
) -> ApiResult<Json<TicketResponse>> {
    let now = Utc::now();
    let ticket_id = Uuid::new_v4();

    // Use project_id from the request
    let project_id = payload.project_id;

    // Get next ticket number for this project
    let max_number: Option<i32> = Ticket::find()
        .filter(ticket::Column::ProjectId.eq(project_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .iter()
        .map(|t| t.ticket_number)
        .max();

    let ticket_number = max_number.unwrap_or(0) + 1;

    // Start transaction
    let txn = state.db.begin().await.map_err(ApiError::from)?;

    // Create ticket
    let ticket = ticket::ActiveModel {
        id: Set(ticket_id),
        project_id: Set(project_id),
        ticket_number: Set(ticket_number),
        title: Set(payload.title.clone()),
        description: Set(payload.description.clone().unwrap_or_default()),
        status: Set(payload.status.clone().unwrap_or_else(|| "backlog".to_string())),
        story_points: Set(payload.story_points),
        epic_id: Set(payload.epic_id),
        parent_id: Set(payload.parent_id),
        created_at: Set(now),
        updated_at: Set(now),
        created_by: Set("system".to_string()), // TODO: Get from auth context
    };

    let result = ticket.insert(&txn).await.map_err(ApiError::from)?;

    // Record creation in ticket_changes
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set(ChangeType::Created.as_str().to_string()),
        field_name: Set(None),
        old_value: Set(None),
        new_value: Set(Some(serde_json::to_string(&result).unwrap())),
        changed_by: Set("system".to_string()),
        changed_at: Set(now),
        message: Set(None),
    };
    change.insert(&txn).await.map_err(ApiError::from)?;

    // Add assignees if provided
    let mut assignees = Vec::new();
    if let Some(assignee_list) = payload.assignees {
        for assignee in assignee_list {
            let assignee_model = ticket_assignee::ActiveModel {
                id: Set(Uuid::new_v4()),
                ticket_id: Set(ticket_id),
                assignee: Set(assignee.clone()),
                assigned_at: Set(now),
                assigned_by: Set("system".to_string()),
            };
            assignee_model.insert(&txn).await.map_err(ApiError::from)?;

            // Record change
            let change = ticket_change::ActiveModel {
                id: Set(Uuid::new_v4()),
                ticket_id: Set(ticket_id),
                change_type: Set(ChangeType::AssigneeAdded.as_str().to_string()),
                field_name: Set(Some("assignee".to_string())),
                old_value: Set(None),
                new_value: Set(Some(assignee.clone())),
                changed_by: Set("system".to_string()),
                changed_at: Set(now),
                message: Set(None),
            };
            change.insert(&txn).await.map_err(ApiError::from)?;

            assignees.push(assignee);
        }
    }

    // Add labels if provided
    let mut labels = Vec::new();
    if let Some(label_list) = payload.labels {
        for label in label_list {
            let label_model = ticket_label::ActiveModel {
                id: Set(Uuid::new_v4()),
                ticket_id: Set(ticket_id),
                label: Set(label.clone()),
                created_at: Set(now),
            };
            label_model.insert(&txn).await.map_err(ApiError::from)?;

            // Record change
            let change = ticket_change::ActiveModel {
                id: Set(Uuid::new_v4()),
                ticket_id: Set(ticket_id),
                change_type: Set(ChangeType::LabelAdded.as_str().to_string()),
                field_name: Set(Some("label".to_string())),
                old_value: Set(None),
                new_value: Set(Some(label.clone())),
                changed_by: Set("system".to_string()),
                changed_at: Set(now),
                message: Set(None),
            };
            change.insert(&txn).await.map_err(ApiError::from)?;

            labels.push(label);
        }
    }

    txn.commit().await.map_err(ApiError::from)?;

    let response = TicketResponse {
        id: result.id.to_string(),
        number: format!("TASK-{}", result.ticket_number),
        title: result.title,
        description: result.description,
        status: result.status,
        story_points: result.story_points,
        assignees,
        labels,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.to_rfc3339(),
        created_by: result.created_by,
        parent_id: result.parent_id.map(|id| id.to_string()),
        epic_id: result.epic_id.map(|id| id.to_string()),
    };

    // Broadcast WebSocket update
    let ws_message = serde_json::to_string(&crate::models::ServerMessage::TicketCreated {
        ticket: response.clone(),
    })
    .unwrap();
    state.ws_state.broadcast(ws_message).await;

    Ok(Json(response))
}

pub async fn get_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<TicketDetailResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    // Get ticket
    let ticket = Ticket::find_by_id(ticket_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    // Get assignees
    let assignees = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|a| a.assignee)
        .collect();

    // Get labels
    let labels = TicketLabel::find()
        .filter(ticket_label::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|l| l.label)
        .collect();

    // Get comments
    let comments = Comment::find()
        .filter(comment::Column::TicketId.eq(ticket_id))
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

    // Get dependencies
    let dependencies = TicketDependency::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut dependency_refs = Vec::new();
    for dep in dependencies {
        if let Some(dep_ticket) = Ticket::find_by_id(dep.depends_on_id)
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

    // Get dependents (tickets that depend on this one)
    let dependents = TicketDependency::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let mut dependent_refs = Vec::new();
    for dep in dependents {
        if let Some(dep_ticket) = Ticket::find_by_id(dep.ticket_id)
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

    // Get linked commits
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

    // Get recent changes
    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .take(10) // Limit to 10 most recent
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

    let ticket_response = TicketResponse {
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
    };

    Ok(Json(TicketDetailResponse {
        ticket: ticket_response,
        comments,
        dependencies: dependency_refs,
        dependents: dependent_refs,
        linked_commits: commits,
        recent_changes: changes,
    }))
}

pub async fn update_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTicketRequest>,
) -> ApiResult<Json<TicketResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let ticket = Ticket::find_by_id(ticket_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    let mut ticket: ticket::ActiveModel = ticket.into();
    let now = Utc::now();

    if let Some(title) = payload.title {
        ticket.title = Set(title);
    }
    if let Some(story_points) = payload.story_points {
        ticket.story_points = Set(Some(story_points));
    }
    if let Some(parent_id) = payload.parent_id {
        ticket.parent_id = Set(Some(parent_id));
    }
    if let Some(epic_id) = payload.epic_id {
        ticket.epic_id = Set(Some(epic_id));
    }

    ticket.updated_at = Set(now);

    let result = ticket
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Get assignees and labels
    let assignees = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|a| a.assignee)
        .collect();

    let labels = TicketLabel::find()
        .filter(ticket_label::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|l| l.label)
        .collect();

    let response = TicketResponse {
        id: result.id.to_string(),
        number: format!("TASK-{}", result.ticket_number),
        title: result.title,
        description: result.description,
        status: result.status,
        story_points: result.story_points,
        assignees,
        labels,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.to_rfc3339(),
        created_by: result.created_by,
        parent_id: result.parent_id.map(|id| id.to_string()),
        epic_id: result.epic_id.map(|id| id.to_string()),
    };

    // Broadcast update
    let ws_message = serde_json::to_string(&crate::models::ServerMessage::TicketUpdated {
        ticket: response.clone(),
    })
    .unwrap();
    state.ws_state.broadcast(ws_message).await;

    Ok(Json(response))
}

pub async fn update_description(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDescriptionRequest>,
) -> ApiResult<Json<TicketResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let ticket = Ticket::find_by_id(ticket_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    let old_description = ticket.description.clone();
    let mut ticket: ticket::ActiveModel = ticket.into();
    let now = Utc::now();

    ticket.description = Set(payload.description.clone());
    ticket.updated_at = Set(now);

    let result = ticket
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Record change
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set(ChangeType::DescriptionChanged.as_str().to_string()),
        field_name: Set(Some("description".to_string())),
        old_value: Set(Some(old_description)),
        new_value: Set(Some(payload.description)),
        changed_by: Set("system".to_string()),
        changed_at: Set(now),
        message: Set(None),
    };
    change
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Get assignees and labels
    let assignees = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|a| a.assignee)
        .collect();

    let labels = TicketLabel::find()
        .filter(ticket_label::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|l| l.label)
        .collect();

    Ok(Json(TicketResponse {
        id: result.id.to_string(),
        number: format!("TASK-{}", result.ticket_number),
        title: result.title,
        description: result.description,
        status: result.status,
        story_points: result.story_points,
        assignees,
        labels,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.to_rfc3339(),
        created_by: result.created_by,
        parent_id: result.parent_id.map(|id| id.to_string()),
        epic_id: result.epic_id.map(|id| id.to_string()),
    }))
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateStatusRequest>,
) -> ApiResult<Json<TicketResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    // Validate status
    TicketStatus::from_str(&payload.status)
        .map_err(|e| ApiError::InvalidInput(e.to_string()))?;

    let ticket = Ticket::find_by_id(ticket_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    let old_status = ticket.status.clone();
    let mut ticket: ticket::ActiveModel = ticket.into();
    let now = Utc::now();

    ticket.status = Set(payload.status.clone());
    ticket.updated_at = Set(now);

    let result = ticket
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Record change
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set(ChangeType::StatusChanged.as_str().to_string()),
        field_name: Set(Some("status".to_string())),
        old_value: Set(Some(old_status.clone())),
        new_value: Set(Some(payload.status.clone())),
        changed_by: Set("system".to_string()),
        changed_at: Set(now),
        message: Set(None),
    };
    change
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Get assignees and labels
    let assignees = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|a| a.assignee)
        .collect();

    let labels = TicketLabel::find()
        .filter(ticket_label::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|l| l.label)
        .collect();

    let response = TicketResponse {
        id: result.id.to_string(),
        number: format!("TASK-{}", result.ticket_number),
        title: result.title,
        description: result.description,
        status: result.status.clone(),
        story_points: result.story_points,
        assignees,
        labels,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.to_rfc3339(),
        created_by: result.created_by,
        parent_id: result.parent_id.map(|id| id.to_string()),
        epic_id: result.epic_id.map(|id| id.to_string()),
    };

    // Broadcast status change
    let ws_message = serde_json::to_string(&crate::models::ServerMessage::StatusChanged {
        ticket_id: result.id.to_string(),
        old_status,
        new_status: payload.status,
    })
    .unwrap();
    state.ws_state.broadcast(ws_message).await;

    Ok(Json(response))
}

pub async fn assign_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<AssignTicketRequest>,
) -> ApiResult<Json<TicketResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let ticket = Ticket::find_by_id(ticket_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    let now = Utc::now();

    // Add assignee
    let assignee = ticket_assignee::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        assignee: Set(payload.assignee.clone()),
        assigned_at: Set(now),
        assigned_by: Set("system".to_string()),
    };
    assignee
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Record change
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set(ChangeType::AssigneeAdded.as_str().to_string()),
        field_name: Set(Some("assignee".to_string())),
        old_value: Set(None),
        new_value: Set(Some(payload.assignee)),
        changed_by: Set("system".to_string()),
        changed_at: Set(now),
        message: Set(None),
    };
    change
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Get updated assignees and labels
    let assignees = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|a| a.assignee)
        .collect();

    let labels = TicketLabel::find()
        .filter(ticket_label::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|l| l.label)
        .collect();

    Ok(Json(TicketResponse {
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
    }))
}

pub async fn unassign_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UnassignTicketRequest>,
) -> ApiResult<Json<TicketResponse>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    let ticket = Ticket::find_by_id(ticket_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    // Find and delete assignee
    let assignee = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .filter(ticket_assignee::Column::Assignee.eq(&payload.assignee))
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound("Assignee not found".to_string()))?;

    let assignee_id = assignee.id;
    TicketAssignee::delete_by_id(assignee_id)
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Record change
    let now = Utc::now();
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set(ChangeType::AssigneeRemoved.as_str().to_string()),
        field_name: Set(Some("assignee".to_string())),
        old_value: Set(Some(payload.assignee)),
        new_value: Set(None),
        changed_by: Set("system".to_string()),
        changed_at: Set(now),
        message: Set(None),
    };
    change
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Get updated assignees and labels
    let assignees = TicketAssignee::find()
        .filter(ticket_assignee::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|a| a.assignee)
        .collect();

    let labels = TicketLabel::find()
        .filter(ticket_label::Column::TicketId.eq(ticket_id))
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .into_iter()
        .map(|l| l.label)
        .collect();

    Ok(Json(TicketResponse {
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
    }))
}

pub async fn delete_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let ticket_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    Ticket::delete_by_id(ticket_id)
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({ "success": true })))
}
