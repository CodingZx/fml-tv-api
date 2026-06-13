use crate::common::consts::lock_keys;
use crate::common::cron::CornTask;
use crate::common::pgcache::dao::{SysCacheDao, SysCounterDao};
use crate::common::pglock::service::DbLock;
use crate::common::state::AppState;
use crate::common::ServerResult;
use async_trait::async_trait;
use std::sync::Arc;

/// 清理过期数据
#[derive(Clone)]
pub struct CacheExpireCleanTask {
    state: Arc<AppState>,
}

impl CacheExpireCleanTask {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl CornTask for CacheExpireCleanTask {
    fn name() -> &'static str {
        "cache_expire_clean"
    }

    fn cron() -> &'static str {
        "0 30 0/1 * * *"
    }

    async fn execute(&self) -> ServerResult<()> {
        let key = lock_keys::cache_clean_key();
        let state = Arc::clone(&self.state);
        DbLock::new(&self.state.db, &key).lock_with_ignore(0, 360, async move || {
            SysCacheDao::new(&state.db).clean_expire_data().await?;
            SysCounterDao::new(&state.db).clean_expire_data().await?;

            Ok(())
        }).await
    }
}
