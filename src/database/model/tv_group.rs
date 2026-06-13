use crate::database::model::VecUUIDs;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type TvGroupActiveModel = ActiveModel;
pub type TvGroupModel = Model;
pub type TvGroupEntity = Entity;
pub type TvGroupColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tv_group")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "types")]
    pub types: VecUUIDs, // 绑定的类型
    #[sea_orm(column_name = "sort_num")]
    pub sort_num: i32,

    #[sea_orm(column_name = "create_time")]
    pub create_time: DateTime,
    #[sea_orm(column_name = "update_time")]
    pub update_time: DateTime,
    #[sea_orm(column_name = "create_user")]
    pub create_user: Uuid,
    #[sea_orm(column_name = "update_user")]
    pub update_user: Uuid,
    #[sea_orm(column_name = "version")]
    pub version: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
