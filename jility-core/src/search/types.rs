use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search filters for advanced ticket search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    /// The search query string
    pub query: String,

    /// Filter by status values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,

    /// Filter by assignees
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,

    /// Filter by labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,

    /// Filter by creator
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    /// Filter by creation date (after)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<Utc>>,

    /// Filter by creation date (before)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_before: Option<DateTime<Utc>>,

    /// Filter by update date (after)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<DateTime<Utc>>,

    /// Filter by update date (before)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_before: Option<DateTime<Utc>>,

    /// Minimum story points
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_points: Option<i32>,

    /// Maximum story points
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_points: Option<i32>,

    /// Filter tickets with comments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_comments: Option<bool>,

    /// Filter tickets with commits
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_commits: Option<bool>,

    /// Filter tickets with dependencies
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_dependencies: Option<bool>,

    /// Filter by epic ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epic_id: Option<Uuid>,

    /// Filter by parent ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,

    /// Filter by project ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,

    /// Search scope (title, description, comments)
    #[serde(default)]
    pub search_in: Vec<String>,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            query: String::new(),
            status: None,
            assignees: None,
            labels: None,
            created_by: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            min_points: None,
            max_points: None,
            has_comments: None,
            has_commits: None,
            has_dependencies: None,
            epic_id: None,
            parent_id: None,
            project_id: None,
            search_in: vec![
                "title".to_string(),
                "description".to_string(),
                "comments".to_string(),
            ],
        }
    }
}

/// Search result with highlighted snippets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Ticket ID
    pub ticket_id: Uuid,

    /// Ticket number (e.g., "TASK-123")
    pub ticket_number: String,

    /// Ticket title
    pub title: String,

    /// Ticket description
    pub description: String,

    /// Ticket status
    pub status: String,

    /// Story points
    pub story_points: Option<i32>,

    /// Highlighted snippet showing match context
    pub snippet: String,

    /// Relevance score (higher is better)
    pub rank: f64,

    /// Where the match was found (title, description, comments)
    pub matched_in: Vec<String>,

    /// Assignees
    #[serde(default)]
    pub assignees: Vec<String>,

    /// Labels
    #[serde(default)]
    pub labels: Vec<String>,

    /// Created by
    pub created_by: String,

    /// Created at
    pub created_at: DateTime<Utc>,

    /// Updated at
    pub updated_at: DateTime<Utc>,

    /// Parent ID (for sub-tasks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,

    /// Epic ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epic_id: Option<Uuid>,
}

/// Paginated search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Search results
    pub results: Vec<SearchResult>,

    /// Total number of results
    pub total: usize,

    /// Whether there are more results
    pub has_more: bool,

    /// Current offset
    pub offset: u64,

    /// Limit used
    pub limit: u64,
}
