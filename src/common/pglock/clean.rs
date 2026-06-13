use crate::common::consts::lock_keys;
use crate::common::cron::CornTask;
use crate::common::pglock::dao::SysLockDao;
use crate::common::pglock::service::DbLock;
use crate::common::state::AppState;
use crate::common::ServerResult;
use async_trait::async_trait;
use std::sync::Arc;

/// 清理过期数据
#[derive(Clone)]
pub struct LockExpireCleanTask {
    state: Arc<AppState>,
}

impl LockExpireCleanTask {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl CornTask for LockExpireCleanTask {
    fn name() -> &'static str {
        "lock_expire_clean"
    }

    fn cron() -> &'static str {
        "0 30 0/1 * * *"
    }

    async fn execute(&self) -> ServerResult<()> {
        let key = lock_keys::lock_clean_key();
        let state = Arc::clone(&self.state);
        DbLock::new(&self.state.db, &key).lock_with_ignore(0, 360, async move || {
            SysLockDao::new(&state.db).clean_expire_data().await?;

            Ok(())
        }).await
    }
}
