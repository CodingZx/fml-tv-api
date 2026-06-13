use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

pub type SysLockActiveModel = ActiveModel;
pub type SysLockModel = Model;
pub type SysLockEntity = Entity;
pub type SysLockColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_lock")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "lock_key")]
    pub lock_key: String, 
    #[sea_orm(column_name = "lock_value")]
    pub lock_value: String, 
    #[sea_orm(column_name = "create_time")]
    pub create_time: DateTime, // 创建时间
    #[sea_orm(column_name = "expire_time")]
    pub expire_time: DateTime, // 过期时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}