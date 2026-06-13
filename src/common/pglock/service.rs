use crate::common::pglock::dao::SysLockDao;
use crate::common::pglock::{AdvisoryLockGuard, LockGuard};
use crate::common::util::sm_util::SMUtil;
use crate::common::util::time_util::TimeUtil;
use crate::common::ServerResult;
use chrono::Local;
use crc::{Crc, CRC_32_ISO_HDLC};
use sea_orm::{DbBackend, DbConn, FromQueryResult, Statement, TransactionTrait};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DbLock<'d> {
    conn: &'d DbConn,
    lock_key: &'d str,
    lock_value: String,
}

impl<'d> DbLock<'d> {
    pub fn new(conn: &'d DbConn, lock_key: &'d str) -> Self {
        let rand_value = format!("{}_{}", Uuid::now_v7(), TimeUtil::format_default(Local::now().naive_local()));
        Self {
            conn,
            lock_key,
            lock_value: SMUtil::sm3_hash(&rand_value),
        }
    }

    /// 尝试获取锁
    /// wait_time: 等待获取锁的最大时间
    /// keep_time: 锁的持有时间(自动过期时间)
    pub async fn acquire(&self, wait_time: u64, keep_time: u64) -> ServerResult<Option<LockGuard>> {
        let expire_time = Local::now().naive_local() + Duration::from_secs(wait_time.into());
        let dao = SysLockDao::new(self.conn);
        loop {
            let id = Uuid::now_v7();
            let locked = dao.create_lock(id, self.lock_key.to_string(), self.lock_value.clone(), keep_time).await?;
            if locked {
                return Ok(Some(self.new_guard(id)))
            }

            let now = Local::now().naive_local();
            if now <= expire_time {
                let _ = sleep(Duration::from_millis(10)).await;
            } else {
                // 已经超过等待时间, 跳出等待
                break;
            }
        }

        Ok(None) // 等待超时，未获取到锁
    }

    /// 释放锁
    pub async fn release(&self, guard: LockGuard) -> ServerResult<()> {
        SysLockDao::new(self.conn).release_lock(guard.id, guard.value.clone()).await?;

        Ok(())
    }

    /// 尝试获得锁并执行闭包, 未获得锁时直接返回OK
    pub async fn lock_with_ignore<F, Fut>(&self, wait_time: u64, keep_time: u64, task: F) -> ServerResult<()>
    where
        F: FnOnce() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ServerResult<()>> + Send + 'static,
    {
        self.lock_with_status(wait_time, keep_time, task).await?;
        Ok(())
    }

    /// 尝试获得锁并执行闭包, 返回获得锁状态
    pub async fn lock_with_status<F, Fut>(&self, wait_time: u64, keep_time: u64, task: F) -> ServerResult<bool>
    where
        F: FnOnce() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ServerResult<()>> + Send + 'static,
    {
        let guard = self.acquire(wait_time, keep_time).await?;
        if let Some(v) = guard {
            match task().await {
                Ok(_) => {
                    self.release(v).await?;
                }
                Err(e) => {
                    self.release(v).await?;
                    return Err(e);
                }
            }
            return Ok(true)
        }
        Ok(false)
    }

    fn new_guard(&self, id: Uuid) -> LockGuard {
        LockGuard::new(id, self.lock_value.to_string())
    }


    /// 尝试获取锁
    /// wait_time: 等待获取锁的最大时间
    /// keep_time: 锁的持有时间(自动过期时间)
    pub async fn advisory_acquire(&self, wait_time: u64) -> ServerResult<Option<AdvisoryLockGuard>> {
        let expire_time = Local::now().naive_local() + Duration::from_secs(wait_time);
        loop {
            let tx = self.conn.begin().await?;
            let statement = Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT pg_try_advisory_xact_lock($1) as status",
                vec![hash_lock_key(&self.lock_key).into()]
            );
            let status = LockStatus::find_by_statement(statement).one(&tx).await?.map(|f|f.status).unwrap_or(false);
            if status {
                return Ok(Some(AdvisoryLockGuard::new(tx)))
            }
            tx.rollback().await?;

            let now = Local::now().naive_local();
            if now <= expire_time {
                let _ = sleep(Duration::from_millis(10)).await;
            } else {
                // 已经超过等待时间, 跳出等待
                break;
            }
        }
        Ok(None) // 等待超时，未获取到锁
    }

    /// 释放锁
    pub async fn advisory_release(&self, mut guard: AdvisoryLockGuard) -> ServerResult<()> {
        if let Some(tx) = guard.tx.take() {
            tx.commit().await?;
        }
        Ok(())
    }

    /// 使用pg advisory lock
    pub async fn advisory_lock<F, Fut>(&self, wait_time: u64, task: F) -> ServerResult<bool>
    where
        F: FnOnce() -> Fut + Sync + Send + 'static,
        Fut: Future<Output = ServerResult<()>> + Send + 'static,
    {
        let guard = self.advisory_acquire(wait_time).await?;
        if let Some(guard) = guard {
            let rs = task().await;
            self.advisory_release(guard).await?;
            if let Err(e) = rs {
                return Err(e);
            }
            return Ok(true);
        }
        Ok(false)
    }
}

fn hash_lock_key(lock_key: &str) -> i64 {
    const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let checksum = CRC.checksum(lock_key.as_bytes());
    (checksum as i64).abs()
}

#[derive(Debug, FromQueryResult)]
struct LockStatus {
    status: bool,
}