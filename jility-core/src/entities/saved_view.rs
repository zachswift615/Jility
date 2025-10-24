use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "saved_views")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// User who created this view
    pub user_id: String,

    /// Display name of the saved view
    pub name: String,

    /// Optional description
    #[sea_orm(nullable)]
    pub description: Option<String>,

    /// Serialized SearchFilters as JSON
    #[sea_orm(column_type = "Text")]
    pub filters: String,

    /// Whether this is the user's default view
    pub is_default: bool,

    /// Whether this view is shared with other users
    pub is_shared: bool,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
