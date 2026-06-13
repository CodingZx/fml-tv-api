use crate::common::pgq::consumer::MessageConsumer;
use crate::common::pgq::service::MessageSender;
use crate::common::state::AppState;
use crate::common::util::collect_util;
use crate::common::{consts, ServerResult};
use crate::consumer::collect_vod_consumer::CollectVodMessage;
use crate::database::dao::collect_site::CollectSiteDao;
use async_trait::async_trait;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// 采集队列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectMessage {
    pub site_id: Uuid,
    pub full: bool,
    pub hour: Option<i64>,
    pub retry: i32,
    pub page: i32,
}


pub(super) struct CollectConsumer {
    state: Arc<AppState>,
}

impl CollectConsumer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl MessageConsumer for CollectConsumer {
    fn queue(&self) -> String {
        consts::queues::COLLECT_QUEUE.to_string()
    }

    fn count(&self) -> u32 {
        5
    }

    async fn consume(&self, msg: &str) -> ServerResult<()> {
        let msg = serde_json::from_str::<CollectMessage>(msg)?;

        let rs = process_collect(self.state.clone(), &msg).await;
        let failed_err = self.state.db.transaction(|tx| {
            Box::pin(async move {
                match rs {
                    Ok(finish) => {
                        if finish {
                            CollectSiteDao::new(tx).update_collect_finish(msg.full, true, msg.site_id).await?;
                        } else {
                            let next_page = msg.next_page();
                            MessageSender::new(tx).send_delay_json_1s(consts::queues::COLLECT_QUEUE, next_page).await?;
                        }
                    },
                    Err(e) => {
                        if msg.retry >= 10 {
                            // 采集失败
                            CollectSiteDao::new(tx).update_collect_finish(msg.full, false, msg.site_id).await?;
                            return Ok(Some(e));
                        }
                        let retry_msg = msg.retry();
                        MessageSender::new(tx).send_delay_json_3s(consts::queues::COLLECT_QUEUE, retry_msg).await?;
                    }
                }
                Ok(None)
            })
        }).await?;

        if let Some(e) = failed_err {
            return Err(e);
        }
        Ok(())
    }
}

async fn process_collect(state: Arc<AppState>, msg: &CollectMessage) -> ServerResult<bool> {
    let site = match CollectSiteDao::new(&state.db).find_undeleted_by_id(msg.site_id).await? {
        Some(site) => site,
        None => return Ok(true),
    };

    let page = msg.page;
    let size = 20;
    let (_, rs) = collect_util::fetch_video_list(&site.req_url, page, size, msg.hour).await?;

    let finish = rs.page >= rs.pagecount;
    let full_collect = msg.full;
    state.db.transaction(|tx| {
        Box::pin(async move {
            let sender = MessageSender::new(tx);
            for vod_info in rs.list {
                let msg = CollectVodMessage {
                    site_id: site.id,
                    vod: vod_info,
                    full: full_collect,
                };
                // 后续处理
                sender.send_json(consts::queues::COLLECT_VOD_QUEUE, msg).await?;
            }
            Ok(())
        })
    }).await?;
    Ok(finish)
}

impl CollectMessage {
    pub fn full(site_id: Uuid) -> Self {
        Self {
            site_id,
            full: true,
            hour: None,
            retry: 0,
            page: 1,
        }
    }
    pub fn hour(site_id: Uuid, hour: i64) -> Self {
        Self {
            site_id,
            full: false,
            hour: Some(hour),
            retry: 0,
            page: 1,
        }
    }
    pub fn next_page(&self) -> Self {
        Self {
            site_id: self.site_id,
            full: self.full,
            hour: self.hour,
            page: self.page + 1,
            retry: 0,
        }
    }

    pub fn retry(&self) -> Self {
        Self {
            site_id: self.site_id,
            full: self.full,
            hour: self.hour,
            page: self.page,
            retry: self.retry + 1,
        }
    }
}