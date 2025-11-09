use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for creating a new ticket
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateTicketParams {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub story_points: Option<i32>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub assignees: Option<Vec<String>>,
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub parent_epic_id: Option<String>,
}

/// Parameters for creating multiple tickets at once
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateTicketsBatchParams {
    pub tickets: Vec<CreateTicketParams>,
    #[serde(default)]
    pub parent_id: Option<String>,
}

/// Parameters for getting a ticket
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetTicketParams {
    pub ticket_id: String,
}

/// Parameters for listing tickets with filters
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListTicketsParams {
    #[serde(default)]
    pub status: Option<Vec<String>>,
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub parent_epic_id: Option<String>,
    #[serde(default)]
    pub unassigned: Option<bool>,
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_limit() -> u64 {
    50
}

/// Parameters for claiming a ticket
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ClaimTicketParams {
    pub ticket_id: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Edit operation types for description updates
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EditOperation {
    ReplaceAll,
    Append,
    Prepend,
    ReplaceLines,
    ReplaceSection,
}

impl std::fmt::Display for EditOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditOperation::ReplaceAll => write!(f, "replace_all"),
            EditOperation::Append => write!(f, "append"),
            EditOperation::Prepend => write!(f, "prepend"),
            EditOperation::ReplaceLines => write!(f, "replace_lines"),
            EditOperation::ReplaceSection => write!(f, "replace_section"),
        }
    }
}

/// Parameters for updating ticket description
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UpdateDescriptionParams {
    pub ticket_id: String,
    pub operation: EditOperation,
    pub content: String,
    #[serde(default)]
    pub start_line: Option<usize>,
    #[serde(default)]
    pub end_line: Option<usize>,
    #[serde(default)]
    pub section_header: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Parameters for updating ticket status
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UpdateStatusParams {
    pub ticket_id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Parameters for adding a comment
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AddCommentParams {
    pub ticket_id: String,
    pub content: String,
}

/// Parameters for assigning a ticket
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssignTicketParams {
    pub ticket_id: String,
    pub assignees: Vec<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Parameters for linking a git commit
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LinkCommitParams {
    pub ticket_id: String,
    pub commit_hash: String,
    #[serde(default)]
    pub commit_message: Option<String>,
}

/// Parameters for adding a dependency
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AddDependencyParams {
    pub ticket_id: String,
    pub depends_on: String,
}

/// Parameters for removing a dependency
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RemoveDependencyParams {
    pub ticket_id: String,
    pub depends_on: String,
}

/// Parameters for getting dependency graph
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetDependencyGraphParams {
    pub ticket_id: String,
}

/// Parameters for searching tickets
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchTicketsParams {
    pub query: String,
    #[serde(default = "default_search_limit")]
    pub limit: u64,
}

fn default_search_limit() -> u64 {
    20
}

/// Parameters for listing templates
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListTemplatesParams {}

/// Parameters for creating from template
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateFromTemplateParams {
    pub template: String,
    pub variables: serde_json::Value,
    #[serde(default)]
    pub assignee: Option<String>,
}
