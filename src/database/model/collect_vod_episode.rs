use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type CollectVodEpiActiveModel = ActiveModel;
pub type CollectVodEpiModel = Model;
pub type CollectVodEpiEntity = Entity;
pub type CollectVodEpiColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collect_vod_episode")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "site_id")]
    pub site_id: Uuid, // 采集站ID
    #[sea_orm(column_name = "collect_vod_id")]
    pub collect_vod_id: Uuid, // 关联视频ID
    #[sea_orm(column_name = "line")]
    pub line: String,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "url")]
    pub url: String,
    #[sea_orm(column_name = "sort_num")]
    pub sort_num: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
