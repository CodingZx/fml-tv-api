use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type CollectVodActiveModel = ActiveModel;
pub type CollectVodModel = Model;
pub type CollectVodEntity = Entity;
pub type CollectVodColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collect_vod")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "site_id")]
    pub site_id: Uuid, // 采集站ID
    #[sea_orm(column_name = "collect_type_id")]
    pub collect_type_id: Uuid, // 关联站点下分类ID
    #[sea_orm(column_name = "vod_id")]
    pub vod_id: String, // 原始视频ID
    #[sea_orm(column_name = "vod_name")]
    pub vod_name: String,
    #[sea_orm(column_name = "vod_pic")]
    pub vod_pic: String, // 视频封面
    #[sea_orm(column_name = "vod_tag")]
    pub vod_tag: String, // 视频标签
    #[sea_orm(column_name = "vod_class")]
    pub vod_class: String, // 视频分类
    #[sea_orm(column_name = "vod_actor")]
    pub vod_actor: String, // 演员
    #[sea_orm(column_name = "vod_blurb")]
    pub vod_blurb: String, // 简介
    #[sea_orm(column_name = "vod_remarks")]
    pub vod_remarks: String,
    #[sea_orm(column_name = "vod_content")]
    pub vod_content: String,
    #[sea_orm(column_name = "create_time")]
    pub create_time: DateTime,
    #[sea_orm(column_name = "episode_count")]
    pub episode_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
