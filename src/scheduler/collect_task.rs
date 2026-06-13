use crate::common::consts::lock_keys;
use crate::common::cron::CornTask;
use crate::common::error::ServerError::OptimisticLock;
use crate::common::pglock::service::DbLock;
use crate::common::pgq::service::MessageSender;
use crate::common::state::AppState;
use crate::common::{consts, logger, ServerResult};
use crate::consumer::collect_consumer::CollectMessage;
use crate::database::dao::collect_site::CollectSiteDao;
use crate::database::model::collect_site::{CollectSiteActiveModel, CollectStatus};
use async_trait::async_trait;
use chrono::Local;
use sea_orm::{Set, TransactionTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct CollectTask {
    state: Arc<AppState>,
}

impl CollectTask {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl CornTask for CollectTask {
    fn name() -> &'static str {
        "collect_task"
    }

    fn cron() -> &'static str {
        "0 10 0/1 * * *"
    }

    async fn execute(&self) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let key = lock_keys::collect_task_key();

        let state = Arc::clone(&self.state);
        DbLock::new(&self.state.db, &key).lock_with_ignore(0, 360, async move || {
            let sites = CollectSiteDao::new(&state.db).find_task_site().await?;
            for site in sites {
                let rs = state.db.transaction(|tx| {
                    Box::pin(async move {
                        let update = CollectSiteActiveModel {
                            collect_status: Set(CollectStatus::Processing),
                            update_time: Set(Local::now().naive_local()),
                            update_user: Set(consts::get_default_id()),
                            ..Default::default()
                        };
                        let rows = CollectSiteDao::new(tx).update_by_version(update, site.id, site.version).await?;
                        if rows == 0 {
                            return Err(OptimisticLock)
                        }

                        let interval = (now - site.last_time).num_hours();

                        let msg = CollectMessage::hour(site.id, interval + 1);
                        MessageSender::new(tx).send_json(consts::queues::COLLECT_QUEUE, msg).await?;
                        Ok(())
                    })
                }).await;
                if let Err(e) = rs {
                    logger::error!("定时任务提交增量采集发生错误, 错误信息:{}", e);
                }
            }
            Ok(())
        }).await?;
        Ok(())
    }
}