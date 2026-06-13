use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type TvVodPicActiveModel = ActiveModel;
pub type TvVodPicModel = Model;
pub type TvVodPicEntity = Entity;
pub type TvVodPicColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tv_vod_pic")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "tv_vod_id")]
    pub tv_vod_id: Uuid,
    #[sea_orm(column_name = "pic")]
    pub pic: String,
    #[sea_orm(column_name = "status")]
    pub status: bool,
    #[sea_orm(column_name = "site_id")]
    pub site_id: Uuid,
    #[sea_orm(column_name = "collect_vod_id")]
    pub collect_vod_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
