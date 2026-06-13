use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type TvTypeActiveModel = ActiveModel;
pub type TvTypeModel = Model;
pub type TvTypeEntity = Entity;
pub type TvTypeColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tv_type")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "sort_num")]
    pub sort_num: i32, // 排序值 小在前

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
