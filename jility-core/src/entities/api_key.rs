use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "api_keys")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    /// Human-readable name for the API key
    pub name: String,

    /// Hashed API key (bcrypt)
    pub key_hash: String,

    /// First 8 characters of the key for identification
    pub prefix: String,

    /// JSON array of scopes (e.g., ["tickets:read", "tickets:write"])
    #[sea_orm(column_type = "Text")]
    pub scopes: String,

    #[sea_orm(nullable)]
    pub expires_at: Option<DateTimeUtc>,

    #[sea_orm(nullable)]
    pub last_used_at: Option<DateTimeUtc>,

    pub created_at: DateTimeUtc,

    #[sea_orm(nullable)]
    pub revoked_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
