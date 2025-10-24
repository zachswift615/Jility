use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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

impl ActiveModelBehavior for ActiveModel {}
