use crate::database::model::HashUUIDs;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type TvVodActiveModel = ActiveModel;
pub type TvVodModel = Model;
pub type TvVodEntity = Entity;
pub type TvVodColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tv_vod")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "clear_name")]
    pub clear_name: String,
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
    #[sea_orm(column_name = "episode_count")]
    pub episode_count: i32, // 剧集数量
    #[sea_orm(column_name = "create_time")]
    pub create_time: DateTime,
    #[sea_orm(column_name = "update_time")]
    pub update_time: DateTime,
    #[sea_orm(column_name = "collect_type")]
    pub collect_type: HashUUIDs,
    #[sea_orm(column_name = "collect_vod")]
    pub collect_vod: HashUUIDs,
    #[sea_orm(column_name = "tv_type")]
    pub tv_type: HashUUIDs,
    #[sea_orm(column_name = "show")]
    pub show: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
