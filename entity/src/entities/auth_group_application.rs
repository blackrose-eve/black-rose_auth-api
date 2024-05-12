//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::GroupApplicationType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "auth_group_application")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
    pub application_type: GroupApplicationType,
    #[sea_orm(column_type = "Text", nullable)]
    pub application_text: Option<String>,
    pub created: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::auth_group::Entity",
        from = "Column::GroupId",
        to = "super::auth_group::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AuthGroup,
    #[sea_orm(
        belongs_to = "super::auth_user::Entity",
        from = "Column::UserId",
        to = "super::auth_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AuthUser,
}

impl Related<super::auth_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthGroup.def()
    }
}

impl Related<super::auth_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}