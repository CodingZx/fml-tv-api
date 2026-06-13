use crate::common::pglock::entity::{SysLockActiveModel, SysLockColumn, SysLockEntity};
use crate::common::ServerResult;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::OnConflict;
use sea_orm::ColumnTrait;
use sea_orm::{ConnectionTrait, EntityTrait, ExprTrait, QueryFilter, Set};
use std::time::Duration;
use uuid::Uuid;

pub struct SysLockDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysLockDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 创建锁
    pub async fn create_lock(&self, id: Uuid, key: String, value: String, expire: u64) -> ServerResult<bool> {
        let now = chrono::Local::now().naive_local();
        let expire = now + Duration::from_secs(expire);
        let model = SysLockActiveModel {
            id: Set(id),
            lock_key: Set(key.clone()),
            lock_value: Set(value.clone()),
            create_time: Set(now),
            expire_time: Set(expire),
        };
        let row = SysLockEntity::insert(model)
            .on_conflict(
                OnConflict::column(SysLockColumn::LockKey)
                    .value(SysLockColumn::CreateTime, now)
                    .value(SysLockColumn::ExpireTime, expire)
                    .value(SysLockColumn::LockValue, value.clone())
                    .action_and_where(Expr::col((super::entity::Entity, SysLockColumn::ExpireTime)).lte(now))
                    .to_owned()
            )
            .exec_without_returning(self.conn)
            .await?;
        Ok(row == 1)
    }

    /// 释放锁
    pub async fn release_lock(&self, lock_id: Uuid, lock_value: String) -> ServerResult<()> {
        SysLockEntity::delete_many()
            .filter(SysLockColumn::Id.eq(lock_id))
            .filter(SysLockColumn::LockValue.eq(lock_value))
            .exec(self.conn)
            .await?;
        Ok(())
    }
    
    /// 清理过期数据
    pub async fn clean_expire_data(&self) -> ServerResult<()> {
        SysLockEntity::delete_many()
            .filter(SysLockColumn::ExpireTime.lt(chrono::Local::now().naive_local()))
            .exec(self.conn)
            .await?;
        Ok(())
    }
}

