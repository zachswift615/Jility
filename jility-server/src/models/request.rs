use serde::Deserialize;
use uuid::Uuid;

// Project requests
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub key: Option<String>,
    pub color: Option<String>,
    #[serde(default)]
    pub ai_planning_enabled: bool,
    #[serde(default)]
    pub auto_link_git: bool,
    #[serde(default)]
    pub require_story_points: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub key: Option<String>,
    pub color: Option<String>,
    pub ai_planning_enabled: Option<bool>,
    pub auto_link_git: Option<bool>,
    pub require_story_points: Option<bool>,
}

// Ticket requests
#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub project_id: Uuid,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub story_points: Option<i32>,
    pub status: Option<String>,
    pub assignees: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub parent_id: Option<Uuid>,
    pub epic_id: Option<Uuid>,
    #[serde(default)]
    pub is_epic: bool,
    pub epic_color: Option<String>,
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

    #[serde(default)]
    pub status: Vec<String>,

    #[serde(default)]
    pub assignees: Vec<String>,

    #[serde(default)]
    pub labels: Vec<String>,

    pub created_by: Option<String>,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,

    pub min_points: Option<i32>,
    pub max_points: Option<i32>,

    pub has_comments: Option<bool>,
    pub has_commits: Option<bool>,
    pub has_dependencies: Option<bool>,

    pub epic_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub project_id: Option<Uuid>,

    #[serde(default)]
    pub search_in: Vec<String>,

    #[serde(default = "default_limit")]
    pub limit: u64,

    #[serde(default)]
    pub offset: u64,
}

fn default_limit() -> u64 {
    20
}

// Saved view requests
#[derive(Debug, Deserialize)]
pub struct CreateSavedViewRequest {
    pub name: String,
    pub description: Option<String>,
    pub filters: serde_json::Value,
    pub is_default: Option<bool>,
    pub is_shared: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSavedViewRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub filters: Option<serde_json::Value>,
    pub is_default: Option<bool>,
    pub is_shared: Option<bool>,
}

// Sprint requests
#[derive(Debug, Deserialize)]
pub struct CreateSprintRequest {
    pub name: String,
    pub goal: Option<String>,
    pub start_date: Option<String>, // ISO 8601 format
    pub end_date: Option<String>,   // ISO 8601 format
    pub capacity: Option<i32>,      // Story point capacity
}

#[derive(Debug, Deserialize)]
pub struct UpdateSprintRequest {
    pub name: Option<String>,
    pub goal: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StartSprintRequest {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTicketToSprintRequest {
    pub added_by: String,
}
