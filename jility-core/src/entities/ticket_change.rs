use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_change")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub ticket_id: Uuid,

    /// Type of change (see ChangeType enum below)
    pub change_type: String,

    /// Field that changed (for field updates)
    #[sea_orm(nullable)]
    pub field_name: Option<String>,

    /// Previous value (JSON-encoded)
    #[sea_orm(column_type = "Text", nullable)]
    pub old_value: Option<String>,

    /// New value (JSON-encoded)
    #[sea_orm(column_type = "Text", nullable)]
    pub new_value: Option<String>,

    /// Who made the change
    pub changed_by: String,

    pub changed_at: DateTimeUtc,

    /// Optional context message (e.g., handoff notes)
    #[sea_orm(column_type = "Text", nullable)]
    pub message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl Related<super::ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ticket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Change types tracked
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    // Lifecycle
    Created,

    // Field updates
    TitleChanged,
    DescriptionChanged,
    StatusChanged,
    StoryPointsChanged,

    // Relationships
    AssigneeAdded,
    AssigneeRemoved,
    LabelAdded,
    LabelRemoved,
    DependencyAdded,
    DependencyRemoved,
    ParentChanged,
    EpicChanged,

    // Collaboration
    CommentAdded,
    CommitLinked,

    // Sprint
    AddedToSprint,
    RemovedFromSprint,
}

#[derive(Debug, Error)]
#[error("Invalid change type: {0}")]
pub struct ChangeTypeError(String);

impl ChangeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::TitleChanged => "title_changed",
            Self::DescriptionChanged => "description_changed",
            Self::StatusChanged => "status_changed",
            Self::StoryPointsChanged => "story_points_changed",
            Self::AssigneeAdded => "assignee_added",
            Self::AssigneeRemoved => "assignee_removed",
            Self::LabelAdded => "label_added",
            Self::LabelRemoved => "label_removed",
            Self::DependencyAdded => "dependency_added",
            Self::DependencyRemoved => "dependency_removed",
            Self::ParentChanged => "parent_changed",
            Self::EpicChanged => "epic_changed",
            Self::CommentAdded => "comment_added",
            Self::CommitLinked => "commit_linked",
            Self::AddedToSprint => "added_to_sprint",
            Self::RemovedFromSprint => "removed_from_sprint",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ChangeTypeError> {
        match s {
            "created" => Ok(Self::Created),
            "title_changed" => Ok(Self::TitleChanged),
            "description_changed" => Ok(Self::DescriptionChanged),
            "status_changed" => Ok(Self::StatusChanged),
            "story_points_changed" => Ok(Self::StoryPointsChanged),
            "assignee_added" => Ok(Self::AssigneeAdded),
            "assignee_removed" => Ok(Self::AssigneeRemoved),
            "label_added" => Ok(Self::LabelAdded),
            "label_removed" => Ok(Self::LabelRemoved),
            "dependency_added" => Ok(Self::DependencyAdded),
            "dependency_removed" => Ok(Self::DependencyRemoved),
            "parent_changed" => Ok(Self::ParentChanged),
            "epic_changed" => Ok(Self::EpicChanged),
            "comment_added" => Ok(Self::CommentAdded),
            "commit_linked" => Ok(Self::CommitLinked),
            "added_to_sprint" => Ok(Self::AddedToSprint),
            "removed_from_sprint" => Ok(Self::RemovedFromSprint),
            _ => Err(ChangeTypeError(s.to_string())),
        }
    }
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
