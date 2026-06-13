use crate::common::consts;
use chrono::Local;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Set};
use serde::{Deserialize, Serialize};

pub type CollectSiteActiveModel = ActiveModel;
pub type CollectSiteModel = Model;
pub type CollectSiteEntity = Entity;
pub type CollectSiteColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collect_site")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "name")]
    pub name: String, // 名称
    #[sea_orm(column_name = "main_page")]
    pub main_page: String, // 主页
    #[sea_orm(column_name = "req_url")]
    pub req_url: String, // 请求URL
    #[sea_orm(column_name = "full_status")]
    pub full_status: CollectStatus, // 全量采集状态
    #[sea_orm(column_name = "full_collect_time")]
    pub full_collect_time: DateTime, // 全量采集完成时间
    #[sea_orm(column_name = "collect_status")]
    pub collect_status: CollectStatus, // 增量采集状态
    #[sea_orm(column_name = "last_time")]
    pub last_time: DateTime, // 最后采集时间
    #[sea_orm(column_name = "status")]
    pub status: bool, // 状态
    #[sea_orm(column_name = "player")]
    pub player: String,

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

    #[sea_orm(column_name = "deleted")]
    pub deleted: bool,
    #[sea_orm(column_name = "delete_time")]
    pub delete_time: DateTime,
    #[sea_orm(column_name = "delete_user")]
    pub delete_user: Uuid,
    #[sea_orm(column_name = "deleted_reason")]
    pub deleted_reason: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl CollectSiteActiveModel {

    pub fn login_delete_created(&mut self) {
        self.deleted = Set(false);
        self.delete_user = Set(consts::get_default_id());
        self.delete_time = Set(consts::get_default_time());
        self.deleted_reason = Set(String::new());
    }

    pub fn login_deleted(&mut self, user: Uuid, reason: String) {
        self.deleted = Set(true);
        self.delete_user = Set(user);
        self.delete_time = Set(Local::now().naive_local());
        self.deleted_reason = Set(reason);
    }
}




/// 采集状态
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum CollectStatus {
    #[sea_orm(num_value = 0)]
    Waiting,
    #[sea_orm(num_value = 1)]
    Processing,
    #[sea_orm(num_value = 2)]
    Completed,
    #[sea_orm(num_value = -2)]
    Failed,
}
impl CollectStatus {
    pub fn get_value(&self) -> i32 {
        match self {
            CollectStatus::Waiting => 0,
            CollectStatus::Processing => 1,
            CollectStatus::Completed => 2,
            CollectStatus::Failed => -2,
        }
    }

    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Waiting),
            1 => Some(Self::Processing),
            2 => Some(Self::Completed),
            -2 => Some(Self::Failed),
            _ => None,
        }
    }
}
