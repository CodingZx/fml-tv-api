use crate::common::cron::CornManager;
use crate::common::state::AppState;
use crate::scheduler::collect_task::CollectTask;
use std::sync::Arc;

pub mod collect_task;

/// 注册定时任务
pub async fn register(register: &mut CornManager, state: Arc<AppState>) {
    let collect_task = CollectTask::new(state.clone());

    register.add_task(collect_task);
}
