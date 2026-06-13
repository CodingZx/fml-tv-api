use crate::database::model::BusinessTypes;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type SysOperLogActiveModel = ActiveModel;
pub type SysOperLogModel = Model;
pub type SysOperLogEntity = Entity;
pub type SysOperLogColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_oper_log")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "title")]
    pub title: String,
    #[sea_orm(column_name = "business_type")]
    pub business_type: BusinessTypes,
    #[sea_orm(column_name = "req_method")]
    pub req_method: String,
    #[sea_orm(column_name = "uri")]
    pub uri: String,
    #[sea_orm(column_name = "exec_time")]
    pub exec_time: i64,
    #[sea_orm(column_name = "req_ip")]
    pub req_ip: String,
    #[sea_orm(column_name = "req_param")]
    pub req_param: String,
    #[sea_orm(column_name = "success")]
    pub success: bool,
    #[sea_orm(column_name = "err_msg")]
    pub err_msg: String,
    #[sea_orm(column_name = "oper_user")]
    pub oper_user: Option<Uuid>,
    #[sea_orm(column_name = "create_time")]
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
