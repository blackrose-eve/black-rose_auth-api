//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "eve_character")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub character_id: i32,
    pub character_name: String,
    pub corporation_id: i32,
    pub last_updated: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auth_user_character_ownership::Entity")]
    AuthUserCharacterOwnership,
    #[sea_orm(
        belongs_to = "super::eve_corporation::Entity",
        from = "Column::CharacterId",
        to = "super::eve_corporation::Column::CorporationId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    EveCorporation,
}

impl Related<super::auth_user_character_ownership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthUserCharacterOwnership.def()
    }
}

impl Related<super::eve_corporation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EveCorporation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
