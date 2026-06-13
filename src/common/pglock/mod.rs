use crate::common::cron::CornManager;
use crate::common::pglock::clean::LockExpireCleanTask;
use crate::common::state::AppState;
use sea_orm::DatabaseTransaction;
use std::sync::Arc;
use uuid::Uuid;

pub mod entity;
pub mod dao;
pub mod service;
pub mod clean;

#[derive(Debug)]
pub struct LockGuard {
    id: Uuid,
    value: String,
}

impl LockGuard {
    fn new(id: Uuid, value: String) -> Self {
        Self { id, value }
    }
}


pub struct AdvisoryLockGuard {
    tx: Option<DatabaseTransaction>,
}

impl AdvisoryLockGuard {
    fn new(tx: DatabaseTransaction) -> Self {
        Self { tx: Some(tx) }
    }
}

impl Drop for AdvisoryLockGuard {
    fn drop(&mut self) {
        // 自动回滚，防止泄露
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            if let Some(tx) = self.tx.take() {
                let _ = handle.block_on(async {
                    let _ = tx.rollback().await;
                });
            }
        }
    }
}


/// 注册定时任务
pub async fn register(register: &mut CornManager, state: Arc<AppState>) {
    let lock_clean_task = LockExpireCleanTask::new(state.clone());

    register.add_task(lock_clean_task);
}
