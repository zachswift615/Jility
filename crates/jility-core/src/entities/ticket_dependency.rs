use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_dependency")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Ticket that has the dependency
    pub ticket_id: Uuid,

    /// Ticket that must be completed first
    pub depends_on_id: Uuid,

    pub created_at: DateTimeUtc,
    pub created_by: String,
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
