use crate::common::pgcache::entity::cache::{SysCacheActiveModel, SysCacheColumn, SysCacheEntity, SysCacheModel};
use crate::common::pgcache::entity::counter::{SysCounterActiveModel, SysCounterColumn, SysCounterEntity, SysCounterModel};
use crate::common::ServerResult;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, ExprTrait, Value};
use sea_orm::{ConnectionTrait, EntityTrait, QueryFilter, Set};
use std::time::Duration;
use uuid::Uuid;

pub struct SysCacheDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysCacheDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    pub async fn insert(&self, key: String, value: String, expire: Duration) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let expire = now + expire;
        let model = SysCacheActiveModel {
            id: Set(Uuid::now_v7()),
            key: Set(key),
            cache: Set(value.clone()),
            create_time: Set(now),
            expire_time: Set(expire),
        };
        SysCacheEntity::insert(model)
            .on_conflict(
                OnConflict::column(SysCacheColumn::Key)
                    .value(SysCacheColumn::CreateTime, now)
                    .value(SysCacheColumn::ExpireTime, expire)
                    .value(SysCacheColumn::Cache, value)
                    .to_owned()
            )
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据缓存Key查询
    pub async fn find_by_key(&self, key: &str) -> ServerResult<Option<SysCacheModel>> {
        let model = SysCacheEntity::find()
            .filter(SysCacheColumn::Key.eq(key))
            .filter(SysCacheColumn::ExpireTime.gt(Local::now().naive_local()))
            .one(self.conn)
            .await?;
        Ok(model)
    }

    /// 更新过期时间
    pub async fn update_expire(&self, key: &str, expire: Duration) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let expire = now + expire;
        SysCacheEntity::update_many()
            .col_expr(SysCacheColumn::ExpireTime, Expr::Value(Value::ChronoDateTime(Some(expire))))
            .filter(SysCacheColumn::Key.eq(key))
            .filter(SysCacheColumn::ExpireTime.gte(now))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 删除缓存
    pub async fn delete_by_key(&self, key: &str) -> ServerResult<()> {
        SysCacheEntity::delete_many()
            .filter(SysCacheColumn::Key.eq(key))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 清理过期数据
    pub async fn clean_expire_data(&self) -> ServerResult<()> {
        SysCacheEntity::delete_many()
            .filter(SysCacheColumn::ExpireTime.lt(Local::now().naive_local()))
            .exec(self.conn)
            .await?;
        Ok(())
    }
}


pub struct SysCounterDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysCounterDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    pub async fn insert(&self, key: String, value: i64, expire: Option<Duration>) -> ServerResult<i64> {
        let now = Local::now().naive_local();
        let expire = match expire {
            Some(v) => now + v,
            None => NaiveDateTime::new(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(), NaiveTime::from_hms_opt(0,0,0).unwrap())
        };
        let model = SysCounterActiveModel {
            id: Set(Uuid::now_v7()),
            key: Set(key),
            counter: Set(value),
            create_time: Set(now),
            expire_time: Set(expire),
        };
        let counter = SysCounterEntity::insert(model)
            .on_conflict(
                OnConflict::column(SysCounterColumn::Key)
                    .value(SysCounterColumn::CreateTime, now)
                    .value(SysCounterColumn::ExpireTime, expire)
                    .value(SysCounterColumn::Counter, Expr::col((super::entity::counter::Entity, SysCounterColumn::Counter)).add(value))
                    .to_owned()
            )
            .exec_with_returning(self.conn)
            .await?;
        Ok(counter.counter)
    }

    /// 根据缓存Key查询
    pub async fn find_by_key(&self, key: &str) -> ServerResult<Option<SysCounterModel>> {
        let model = SysCounterEntity::find()
            .filter(SysCounterColumn::Key.eq(key))
            .filter(SysCounterColumn::ExpireTime.gt(Local::now().naive_local()))
            .one(self.conn)
            .await?;
        Ok(model)
    }

    /// 更新过期时间
    pub async fn update_expire(&self, key: &str, expire: Duration) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let expire = now + expire;
        SysCounterEntity::update_many()
            .col_expr(SysCounterColumn::ExpireTime, Expr::Value(Value::ChronoDateTime(Some(expire))))
            .filter(SysCounterColumn::Key.eq(key))
            .filter(SysCounterColumn::ExpireTime.gte(now))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 删除缓存
    pub async fn delete_by_key(&self, key: &str) -> ServerResult<()> {
        SysCounterEntity::delete_many()
            .filter(SysCounterColumn::Key.eq(key))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 删除缓存
    pub async fn delete_by_keys(&self, keys: impl IntoIterator<Item = String>) -> ServerResult<()> {
        SysCounterEntity::delete_many()
            .filter(SysCounterColumn::Key.is_in(keys))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 清理过期数据
    pub async fn clean_expire_data(&self) -> ServerResult<()> {
        SysCounterEntity::delete_many()
            .filter(SysCounterColumn::ExpireTime.lt(Local::now().naive_local()))
            .exec(self.conn)
            .await?;
        Ok(())
    }
}

