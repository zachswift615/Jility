use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub project_id: Uuid,

    /// Auto-incrementing ticket number within project (e.g., 1, 2, 3...)
    /// Display as "TASK-1", "TASK-2", etc.
    pub ticket_number: i32,

    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub description: String,

    /// Current status (stored as string, converted to enum in code)
    pub status: String, // "backlog", "todo", "in_progress", "review", "done", "blocked"

    #[sea_orm(nullable)]
    pub story_points: Option<i32>,

    /// Reference to epic (large feature)
    #[sea_orm(nullable)]
    pub epic_id: Option<Uuid>,

    /// Reference to parent ticket (for sub-tasks)
    #[sea_orm(nullable)]
    pub parent_id: Option<Uuid>,

    /// Whether this ticket is an epic (large feature container)
    #[serde(default)]
    pub is_epic: bool,

    /// Optional color for epic visualization (hex color code)
    #[sea_orm(nullable)]
    pub epic_color: Option<String>,

    /// Reference to parent epic (for tickets belonging to an epic)
    /// Note: This serves the same purpose as epic_id but was added by migration
    #[sea_orm(nullable)]
    pub parent_epic_id: Option<Uuid>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    /// Who created this ticket ("agent-1", "alice", etc.)
    pub created_by: String,

    /// Soft delete timestamp - if set, ticket is considered deleted
    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,

    #[sea_orm(has_many = "super::ticket_assignee::Entity")]
    Assignees,

    #[sea_orm(has_many = "super::ticket_label::Entity")]
    Labels,

    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,

    #[sea_orm(has_many = "super::ticket_change::Entity")]
    Changes,

    #[sea_orm(has_many = "super::ticket_dependency::Entity")]
    Dependencies,

    #[sea_orm(has_many = "super::commit_link::Entity")]
    CommitLinks,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::ticket_assignee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assignees.def()
    }
}

impl Related<super::ticket_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Labels.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::ticket_change::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Changes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Rust enum for status (converted to/from string in DB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    Review,
    Done,
    Blocked,
}

#[derive(Debug, Error)]
#[error("Invalid ticket status: {0}")]
pub struct TicketStatusError(String);

impl TicketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Backlog => "backlog",
            Self::Todo => "todo",
            Self::InProgress => "in_progress",
            Self::Review => "review",
            Self::Done => "done",
            Self::Blocked => "blocked",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, TicketStatusError> {
        match s {
            "backlog" => Ok(Self::Backlog),
            "todo" => Ok(Self::Todo),
            "in_progress" => Ok(Self::InProgress),
            "review" => Ok(Self::Review),
            "done" => Ok(Self::Done),
            "blocked" => Ok(Self::Blocked),
            _ => Err(TicketStatusError(s.to_string())),
        }
    }
}

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
