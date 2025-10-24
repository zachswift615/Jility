use serde::Deserialize;
use uuid::Uuid;

// Project requests
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

// Ticket requests
#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub story_points: Option<i32>,
    pub status: Option<String>,
    pub assignees: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub parent_id: Option<Uuid>,
    pub epic_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub story_points: Option<i32>,
    pub parent_id: Option<Uuid>,
    pub epic_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDescriptionRequest {
    pub description: String,
    pub operation: Option<String>, // "replace_all", "append", etc.
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignTicketRequest {
    pub assignee: String,
}

#[derive(Debug, Deserialize)]
pub struct UnassignTicketRequest {
    pub assignee: String,
}

// Comment requests
#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

// Dependency requests
#[derive(Debug, Deserialize)]
pub struct AddDependencyRequest {
    pub depends_on_id: Uuid,
}

// Git integration requests
#[derive(Debug, Deserialize)]
pub struct LinkCommitRequest {
    pub commit_hash: String,
    pub commit_message: Option<String>,
}

// Search request
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<u64>,
}
