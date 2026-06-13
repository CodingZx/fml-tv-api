use crate::common::consts;
use chrono::Local;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Set};

pub type SysAccountActiveModel = ActiveModel;
pub type SysAccountModel = Model;
pub type SysAccountEntity = Entity;
pub type SysAccountColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_account")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "username")]
    pub username: String, // 登录账号
    #[sea_orm(column_name = "real_name")]
    pub real_name: String, // 真实姓名
    #[sea_orm(column_name = "password")]
    pub password: String, // 密码
    #[sea_orm(column_name = "status")]
    pub status: bool, // 状态
    #[sea_orm(column_name = "super_admin")]
    pub super_admin: bool, // 是否超级管理员

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

impl SysAccountActiveModel {

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
