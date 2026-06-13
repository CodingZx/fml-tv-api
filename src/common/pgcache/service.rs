use crate::common::pgcache::dao::{SysCacheDao, SysCounterDao};
use crate::common::pglock::service::DbLock;
use crate::common::ServerResult;
use sea_orm::{ConnectionTrait, DbConn};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

// None占位符
const NONE_PLACEHOLDER: &str = "*";

pub struct DbOpsCache<'d> {
    conn: &'d DbConn,
    key: &'d str,
}

impl<'d> DbOpsCache<'d> {
    pub fn new(conn: &'d DbConn, key: &'d str) -> Self {
        Self { conn, key }
    }

    /// 获取单个缓存, 如果数据存在, 重置过期时间
    pub async fn expire_get<T, F, Fut>(&self, time: Duration, execute: F) -> ServerResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Option<T>>> + Send,
    {
        self.lock_get(true, time, execute).await
    }


    /// 获取单个缓存, 如果数据存在, 直接返回, 不重置过期时间
    pub async fn no_expire_get<T, F, Fut>(&self, time: Duration, execute: F) -> ServerResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Option<T>>> + Send,
    {
        self.lock_get(false, time, execute).await
    }


    /// 加锁获取单个缓存
    async fn lock_get<T, F, Fut>(&self, expire: bool, time: Duration, execute: F) -> ServerResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Option<T>>> + Send,
    {
        let (placeholder, value) = self.get_obj_data(expire, time).await?;
        // 数据已存在, 直接返回, 不需要加锁
        if let Some(v) = value {
            return Ok(Some(v));
        }
        if placeholder {
            return Ok(None);
        }

        let lock_key = format!("{}:load:lock", self.key);
        let lock_srv = DbLock::new(self.conn, &lock_key);

        loop {
            let guard = lock_srv.acquire(0, 360).await?;
            if let Some(v) = guard {
                // 执行获取
                let res = self.inner_get(expire, time, execute).await;
                // release
                lock_srv.release(v).await?;
                return res;
            } else {
                let _ = sleep(Duration::from_millis(10)).await;
            }
        }
    }

    /// 获得单个缓存, 如果数据存在则返回, 如果数据不存在, 则执行Fn并放入缓存
    async fn inner_get<T, F, Fut>(&self, expire: bool, time: Duration, execute: F) -> ServerResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Option<T>>> + Send,
    {
        let (placeholder, value) = self.get_obj_data(expire, time).await?;

        // 数据已存在, 返回
        if let Some(v) = value {
            return Ok(Some(v));
        }
        // 数据不存在, 如果为占位符, 直接返回None
        if placeholder {
            return Ok(None);
        }

        // 执行Fn
        let data = execute().await?;

        self.inner_set_data(&data, time).await?;
        Ok(data)
    }

    /// 设置缓存数据
    async fn inner_set_data<T: Serialize>(&self, data: &Option<T>, time: Duration) -> ServerResult<()> {
        // 如果返回数据为空则写入占位符
        let serialized = if let Some(value) = &data {
            serde_json::to_string(value)?
        } else {
            NONE_PLACEHOLDER.to_string()
        };
        DbCache::new(self.conn, &self.key).set_str(&serialized, time).await?;
        Ok(())
    }

    /// 读取Redis数据, 如果数据为占位符, 则bool返回true
    async fn get_obj_data<T>(&self, expire: bool, time: Duration) -> ServerResult<(bool, Option<T>)>
    where
        T: DeserializeOwned,
    {
        let ops = DbCache::new(self.conn, &self.key);
        let value = ops.get_str().await?;
        match value {
            None => Ok((false, None)),
            Some(str) => {
                if str == NONE_PLACEHOLDER {
                    ops.expire(time).await?;
                    return Ok((true, None));
                }
                // 是否重置过期时间
                if expire {
                    ops.expire(time).await?;
                }
                let parsed: T = serde_json::from_str(&str)?;
                Ok((false, Some(parsed)))
            }
        }
    }

    /// 获取List缓存, 如果数据存在, 重置过期时间
    pub async fn expire_get_list<T, F, Fut>(&self, time: Duration, execute: F) -> ServerResult<Vec<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Vec<T>>> + Send,
    {
        self.lock_get_list(true, time, execute).await
    }

    /// 获取List缓存
    async fn lock_get_list<T, F, Fut>(&self, expire: bool, time: Duration, execute: F) -> ServerResult<Vec<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Vec<T>>> + Send,
    {
        let value= self.get_list_data(expire, time).await?;

        // 数据已存在, 直接返回, 不需要加锁
        if let Some(v) = value {
            return Ok(v);
        }

        let lock_key = format!("{}:load:lock", self.key);
        let lock_srv = DbLock::new(self.conn, &lock_key);

        loop {
            let guard = lock_srv.acquire(0, 360).await?;
            if let Some(v) = guard {
                // 执行获取
                let res = self.inner_get_list(expire, time, execute).await;
                lock_srv.release(v).await?;
                return res;
            } else {
                let _ = sleep(Duration::from_millis(10)).await;
            }
        }
    }

    async fn inner_get_list<T, F, Fut>(&self, expire: bool, time: Duration, execute: F) -> ServerResult<Vec<T>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = ServerResult<Vec<T>>> + Send,
    {
        let redis_val: Option<Vec<T>> = self.get_list_data(expire, time).await?;

        // 数据已存在, 直接返回
        if let Some(v) = redis_val {
            return Ok(v);
        }
        // 数据不存在
        let data = execute().await?;
        DbCache::new(self.conn, &self.key).set_json(&data, time).await?;

        Ok(data)
    }

    /// 读取数据
    async fn get_list_data<T>(&self, expire: bool, time: Duration) -> ServerResult<Option<Vec<T>>>
    where T: DeserializeOwned,
    {
        let ops = DbCache::new(self.conn, &self.key);
        let cache_value = ops.get_str().await?;
        match cache_value {
            None => Ok(None),
            Some(str) => {
                // 是否重置过期时间
                if expire {
                    ops.expire(time).await?;
                }
                let parsed: Vec<T> = serde_json::from_str(&str)?;
                Ok(Some(parsed))
            }
        }
    }
}


pub struct DbCache<'d, C: ConnectionTrait> {
    conn: &'d C,
    key: &'d str,
}

impl<'d, C: ConnectionTrait> DbCache<'d, C> {
    pub fn new(conn: &'d C, key: &'d str) -> Self {
        Self { conn, key }
    }

    /// 写入数据
    pub async fn set_str(&self, value: &str, time: Duration) -> ServerResult<()> {
        SysCacheDao::new(self.conn).insert(self.key.to_string(), value.to_string(), time).await?;
        Ok(())
    }

    /// 获得缓存的值
    pub async fn get_str(&self) -> ServerResult<Option<String>> {
        let cache = SysCacheDao::new(self.conn).find_by_key(self.key).await?.map(|r|r.cache);
        Ok(cache)
    }

    /// 写入数据
    pub async fn set_json<T: Serialize>(&self, val: &T, time: Duration) -> ServerResult<()> {
        let value = serde_json::to_string(val)?;
        self.set_str(&value, time).await?;

        Ok(())
    }

    /// 获得缓存值
    pub async fn get_json<T: DeserializeOwned>(&self) -> ServerResult<Option<T>> {
        let cache = SysCacheDao::new(self.conn).find_by_key(self.key).await?;
        let cache_value = match cache {
            None => return Ok(None),
            Some(v) => v.cache,
        };

        Ok(serde_json::from_str(&cache_value)?)
    }

    /// 设置过期时间
    pub async fn expire(&self, time: Duration) -> ServerResult<()> {
        SysCacheDao::new(self.conn).update_expire(self.key, time).await?;
        Ok(())
    }

    /// 删除KEY
    pub async fn delete(&self) -> ServerResult<()> {
        SysCacheDao::new(self.conn).delete_by_key(self.key).await?;
        Ok(())
    }
}

pub struct DbCounter<'d, C: ConnectionTrait> {
    conn: &'d C,
    key: String,
}

impl<'d, C: ConnectionTrait> DbCounter<'d, C> {

    pub fn new(conn: &'d C, key: String) -> Self {
        Self { conn, key }
    }

    /// Incr
    pub async fn incr(&self, val: i64, expire: Duration) -> ServerResult<i64> {
        let counter = SysCounterDao::new(self.conn).insert(self.key.clone(), val.into(), Some(expire)).await?;
        Ok(counter)
    }

    /// Incr
    pub async fn incr_no_time(&self, val: i64) -> ServerResult<i64> {
        let counter = SysCounterDao::new(self.conn).insert(self.key.clone(), val.into(), None).await?;
        Ok(counter)
    }

    /// Decr
    pub async fn decr(&self, val: i64, expire: Duration) -> ServerResult<i64> {
        let counter = SysCounterDao::new(self.conn).insert(self.key.clone(), -val, Some(expire)).await?;
        Ok(counter)
    }

    /// Decr
    pub async fn decr_no_time(&self, val: i64) -> ServerResult<i64> {
        let counter = SysCounterDao::new(self.conn).insert(self.key.clone(), -val, None).await?;
        Ok(counter)
    }

    /// 获得缓存值
    pub async fn get_num(&self) -> ServerResult<Option<i64>> {
        let cache = SysCounterDao::new(self.conn).find_by_key(&self.key).await?;
        let cache_value = match cache {
            None => None,
            Some(v) => Some(v.counter),
        };

        Ok(cache_value)
    }

    /// 设置过期时间
    pub async fn expire(&self, time: Duration) -> ServerResult<()> {
        SysCounterDao::new(self.conn).update_expire(&self.key, time).await?;
        Ok(())
    }

    /// 删除KEY
    pub async fn delete(&self) -> ServerResult<()> {
        SysCounterDao::new(self.conn).delete_by_key(&self.key).await?;

        Ok(())
    }
}

pub struct DbCacheClear<'d, C: ConnectionTrait> {
    conn: &'d C,
    key: Vec<String>,
}

impl<'d, C: ConnectionTrait> DbCacheClear<'d, C> {

    pub fn from_keys(conn: &'d C, keys: impl IntoIterator<Item = String>) -> Self {
        let key = keys.into_iter().collect();
        Self { conn, key, }
    }

    pub fn from_key(conn: &'d C, key: String) -> Self {
        let key = vec![key];
        Self { conn, key, }
    }

    /// 删除KEY
    pub async fn delete(&self) -> ServerResult<()> {
        SysCounterDao::new(self.conn).delete_by_keys(self.key.clone()).await?;
        Ok(())
    }
}