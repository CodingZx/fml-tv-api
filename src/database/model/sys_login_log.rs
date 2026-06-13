use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type SysLoginLogActiveModel = ActiveModel;
pub type SysLoginLogModel = Model;
pub type SysLoginLogEntity = Entity;
pub type SysLoginLogColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_login_log")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "ip_addr")]
    pub ip_addr: String,
    #[sea_orm(column_name = "user_name")]
    pub user_name: String, // 登录账号
    #[sea_orm(column_name = "password")]
    pub password: String, // 密码

    #[sea_orm(column_name = "create_time")]
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

