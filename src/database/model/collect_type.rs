use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type CollectTypeActiveModel = ActiveModel;
pub type CollectTypeModel = Model;
pub type CollectTypeEntity = Entity;
pub type CollectTypeColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collect_type")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "site_id")]
    pub site_id: Uuid, // 采集站ID
    #[sea_orm(column_name = "type_id")]
    pub type_id: i32, // 分类ID
    #[sea_orm(column_name = "type_name")]
    pub type_name: String, // 分类名称
    #[sea_orm(column_name = "vod_count")]
    pub vod_count: i32, // 剧集数量
    #[sea_orm(column_name = "show")]
    pub show: bool, // 是否显示
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
