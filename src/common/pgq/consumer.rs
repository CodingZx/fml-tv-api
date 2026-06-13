use crate::common::pgq::dao::{SysQueueDelayMsgDao, SysQueueMsgDao};
use crate::common::pgq::entity::message::SysQueueMsgActiveModel;
use crate::common::pgq::MessageStatus;
use crate::common::state::AppState;
use crate::common::{consts, logger, ServerResult};
use async_trait::async_trait;
use chrono::Local;
use sea_orm::{Set, TransactionTrait};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// 消费者
#[async_trait]
pub trait MessageConsumer: Sync + Send + 'static {
    /// 监听队列
    fn queue(&self) -> String;

    /// 启动消费个数
    fn count(&self) -> u32;

    /// 消费消息
    async fn consume(&self, msg: &str) -> ServerResult<()>;
}

/// 消费者注册器
pub struct ConsumerRegister {
    state: Arc<AppState>,
    consumers: Vec<Arc<dyn MessageConsumer>>,
}

impl ConsumerRegister {

    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            consumers: vec![],
        }
    }

    /// 注册消费者
    pub fn register<C: MessageConsumer>(&mut self, consumer: C) {
        self.consumers.push(Arc::new(consumer));
    }

    /// 启动所有消费者
    pub async fn start(&self) -> ServerResult<()> {
        if !self.state.pgq_conf.enable {
            return Ok(())
        }
        // 启动延迟队列监听
        {
            let state = Arc::clone(&self.state);
            tokio::spawn(async move {
                Self::start_delay_observer(Arc::clone(&state)).await.expect("延迟队列监听启动失败");
            });
        }

        // 启动注册的消费者
        for consumer in self.consumers.iter() {
            let consumer = Arc::clone(&consumer);
            for _ in 0..consumer.count() {
                let state = Arc::clone(&self.state);
                let consumer = Arc::clone(&consumer);
                tokio::spawn(async move {
                    let state = Arc::clone(&state);
                    let consumer = Arc::clone(&consumer);
                    let queue = consumer.queue();
                    Self::start_consumer(state, consumer)
                        .await
                        .expect(&format!("队列[{queue}]的消费者启动失败"));
                });
            }
        }

        Ok(())
    }

    async fn start_consumer(state: Arc<AppState>, consumer: Arc<dyn MessageConsumer>) -> ServerResult<()> {
        let auto_clean = state.pgq_conf.auto_clean;
        loop {
            let wait = state.db.transaction(|tx| {
                let consumer = Arc::clone(&consumer);
                Box::pin(async move {
                    let msg_dao = SysQueueMsgDao::new(tx);
                    let msg = msg_dao.take_one_msg(&consumer.queue()).await?;
                    match msg {
                        None => Ok(true),
                        Some(v) => {
                            msg_dao.update_processing(v.id).await?;
                            match consumer.consume(&v.message).await {
                                Ok(_) => {
                                    if auto_clean {
                                        msg_dao.delete_by_id(v.id).await?;
                                    } else {
                                        msg_dao.update_success(v.id).await?;
                                    }
                                }
                                Err(e) => {
                                    // 记录错误，但不让它中断消费者
                                    if let Err(db_err) = msg_dao.update_failed(v.id, e.to_string()).await {
                                        logger::error!("Failed to update message status: {}", db_err);
                                    }
                                }
                            }
                            Ok(false)
                        }
                    }
                })
            }).await?;
            if wait {
                tokio::time::sleep(Duration::from_millis(state.pgq_conf.fetch_interval_ms)).await;
            }
        }
    }

    /// 启动监听延迟队列
    async fn start_delay_observer(state: Arc<AppState>) -> ServerResult<()> {
        loop {
            let default_time = consts::get_default_time();

            let wait = state.db.transaction(|tx|{
                Box::pin(async move {
                    let delay_dao = SysQueueDelayMsgDao::new(tx);
                    let msg_dao = SysQueueMsgDao::new(tx);
                    let delay_messages = delay_dao.take_delay_msg().await?;
                    let mut messages = Vec::new();
                    let mut delay_ids = Vec::new();
                    for delay_message in delay_messages.into_iter() {
                        messages.push(SysQueueMsgActiveModel {
                            id: Set(Uuid::now_v7()),
                            queue: Set(delay_message.queue),
                            message: Set(delay_message.message),
                            status: Set(MessageStatus::Pending),
                            publish_time: Set(Local::now().naive_local()),
                            process_time: Set(default_time),
                            finish_time: Set(default_time),
                            error_detail: Set(String::new()),
                        });
                        delay_ids.push(delay_message.id);
                    }
                    if !messages.is_empty() {
                        msg_dao.batch_insert(messages).await?;
                        delay_dao.delete_by_ids(delay_ids).await?;
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                })
            }).await?;
            if wait {
                tokio::time::sleep(Duration::from_millis(state.pgq_conf.delay_fetch_interval_ms)).await;
            }
        }
    }
}