use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub workspace_id: Uuid,

    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    /// Project key/prefix for ticket IDs (e.g., "PROJ" for PROJ-123)
    pub key: Option<String>,

    /// Hex color code for project UI (e.g., "#5e6ad2")
    pub color: Option<String>,

    /// Enable AI planning features
    #[sea_orm(default_value = false)]
    pub ai_planning_enabled: bool,

    /// Automatically link commits that mention ticket IDs
    #[sea_orm(default_value = false)]
    pub auto_link_git: bool,

    /// Require story points before moving tickets to In Progress
    #[sea_orm(default_value = false)]
    pub require_story_points: bool,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::Id",
        on_delete = "Cascade"
    )]
    Workspace,

    #[sea_orm(has_many = "super::ticket::Entity")]
    Tickets,

    #[sea_orm(has_many = "super::sprint::Entity")]
    Sprints,
}

impl Related<super::ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tickets.def()
    }
}

impl Related<super::sprint::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sprints.def()
    }
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
