use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// Project responses
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub key: Option<String>,
    pub color: Option<String>,
    pub ai_planning_enabled: bool,
    pub auto_link_git: bool,
    pub require_story_points: bool,
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
    pub user_name: String,
    pub changed_at: String,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HistoryVersionResponse {
    pub version: i32,
    pub description: String,
    pub user_name: String,
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

// Sprint responses
#[derive(Debug, Serialize, Clone)]
pub struct SprintResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub goal: Option<String>,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct SprintDetailsResponse {
    pub sprint: SprintResponse,
    pub tickets: Vec<TicketResponse>,
    pub stats: SprintStats,
}

#[derive(Debug, Serialize, Clone)]
pub struct SprintStats {
    pub total_tickets: usize,
    pub total_points: i32,
    pub completed_tickets: usize,
    pub completed_points: i32,
    pub in_progress_tickets: usize,
    pub in_progress_points: i32,
    pub todo_tickets: usize,
    pub todo_points: i32,
    pub completion_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct BurndownDataPoint {
    pub date: String,
    pub ideal: i32,
    pub actual: i32,
}

#[derive(Debug, Serialize)]
pub struct BurndownData {
    pub sprint_id: String,
    pub data_points: Vec<BurndownDataPoint>,
}

#[derive(Debug, Serialize)]
pub struct VelocityData {
    pub sprint_name: String,
    pub completed_points: i32,
}

#[derive(Debug, Serialize)]
pub struct SprintHistoryResponse {
    pub sprints: Vec<SprintResponse>,
    pub velocity_data: Vec<VelocityData>,
    pub average_velocity: f64,
}

// Saved view responses
#[derive(Debug, Serialize)]
pub struct SavedViewResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub filters: serde_json::Value,
    pub is_default: bool,
    pub is_shared: bool,
    pub created_at: String,
    pub updated_at: String,
}
