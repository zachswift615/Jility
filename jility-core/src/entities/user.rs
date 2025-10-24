use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub email: String,

    #[sea_orm(unique)]
    pub username: String,

    /// Hashed password (bcrypt)
    pub password_hash: String,

    #[sea_orm(nullable)]
    pub full_name: Option<String>,

    #[sea_orm(nullable)]
    pub avatar_url: Option<String>,

    pub is_active: bool,
    pub is_verified: bool,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(nullable)]
    pub last_login: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_key::Entity")]
    ApiKeys,

    #[sea_orm(has_many = "super::session::Entity")]
    Sessions,
}

impl Related<super::api_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKeys.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
