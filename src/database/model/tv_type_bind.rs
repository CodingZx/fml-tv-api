use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type TvTypeBindActiveModel = ActiveModel;
pub type TvTypeBindModel = Model;
pub type TvTypeBindEntity = Entity;
pub type TvTypeBindColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tv_type_bind")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "tv_type_id")]
    pub tv_type_id: Uuid,
    #[sea_orm(column_name = "collect_type_id")]
    pub collect_type_id: Uuid,
    #[sea_orm(column_name = "site_id")]
    pub site_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
