use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sprints")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub project_id: Uuid,

    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub goal: Option<String>,

    #[sea_orm(nullable)]
    pub start_date: Option<DateTimeUtc>,

    #[sea_orm(nullable)]
    pub end_date: Option<DateTimeUtc>,

    /// "planning", "active", "completed"
    pub status: String,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,

    #[sea_orm(has_many = "super::sprint_ticket::Entity")]
    SprintTickets,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::sprint_ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SprintTickets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
