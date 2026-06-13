use crate::common::cron::CornManager;
use crate::common::pgcache::clean::CacheExpireCleanTask;
use crate::common::state::AppState;
use std::sync::Arc;

pub mod entity;
pub mod dao;
pub mod service;
pub mod clean;


/// 注册定时任务
pub async fn register(register: &mut CornManager, state: Arc<AppState>) {
    let cache_clean_task = CacheExpireCleanTask::new(state.clone());

    register.add_task(cache_clean_task);
}
