use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// Project responses
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Ticket responses
#[derive(Debug, Serialize, Clone)]
pub struct TicketResponse {
    pub id: String,
    pub number: String, // e.g., "TASK-1"
    pub title: String,
    pub description: String,
    pub status: String,
    pub story_points: Option<i32>,
    pub assignees: Vec<String>,
    pub labels: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
    pub parent_id: Option<String>,
    pub epic_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TicketDetailResponse {
    pub ticket: TicketResponse,
    pub comments: Vec<CommentResponse>,
    pub dependencies: Vec<TicketReference>,
    pub dependents: Vec<TicketReference>,
    pub linked_commits: Vec<CommitLinkResponse>,
    pub recent_changes: Vec<ChangeEventResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TicketReference {
    pub id: String,
    pub number: String,
    pub title: String,
    pub status: String,
}

// Comment responses
#[derive(Debug, Serialize, Clone)]
pub struct CommentResponse {
    pub id: String,
    pub ticket_id: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

// Dependency responses
#[derive(Debug, Serialize)]
pub struct DependencyGraphResponse {
    pub ticket: TicketReference,
    pub dependencies: Vec<TicketReference>,
    pub dependents: Vec<TicketReference>,
}

// Activity & History
#[derive(Debug, Serialize, Clone)]
pub struct ChangeEventResponse {
    pub id: String,
    pub change_type: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: String,
    pub changed_at: String,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HistoryVersionResponse {
    pub version: i32,
    pub description: String,
    pub changed_by: String,
    pub changed_at: String,
}

// Git integration
#[derive(Debug, Serialize, Clone)]
pub struct CommitLinkResponse {
    pub id: String,
    pub commit_hash: String,
    pub commit_message: Option<String>,
    pub linked_at: String,
    pub linked_by: String,
}

// WebSocket messages
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    TicketUpdated { ticket: TicketResponse },
    TicketCreated { ticket: TicketResponse },
    StatusChanged {
        ticket_id: String,
        old_status: String,
        new_status: String,
    },
    CommentAdded {
        ticket_id: String,
        comment: CommentResponse,
    },
    DescriptionEdited {
        ticket_id: String,
        version: i32,
    },
}

// Utility function to format UUID as string
pub fn format_uuid(uuid: &Uuid) -> String {
    uuid.to_string()
}

// Utility function to format DateTime
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}
