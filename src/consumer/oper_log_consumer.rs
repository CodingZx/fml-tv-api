use crate::common::pgq::consumer::MessageConsumer;
use crate::common::state::AppState;
use crate::common::{consts, ServerResult};
use crate::database::dao::sys_oper_log::SysOperLogDao;
use crate::database::model::sys_oper_log::SysOperLogActiveModel;
use crate::database::model::BusinessTypes;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// 操作日志队列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperLogMessage {
    pub title: String,
    pub bis_type: BusinessTypes,
    pub method: String,
    pub uri: String,
    pub exec_time: i64,
    pub req_ip: String,
    pub req_param: String,
    pub oper_user: Option<Uuid>,
    pub success: bool,
    pub error_msg: String,
    pub create_time: NaiveDateTime,
}

/// 操作日志消费者
pub(super) struct OperLogConsumer {
    state: Arc<AppState>,
}

impl OperLogConsumer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl MessageConsumer for OperLogConsumer {
    fn queue(&self) -> String {
        consts::queues::OPER_LOG_QUEUE.to_string()
    }

    fn count(&self) -> u32 {
        1
    }

    async fn consume(&self, msg: &str) -> ServerResult<()> {
        let msg: OperLogMessage = serde_json::from_str(msg)?;

        let log = SysOperLogActiveModel {
            id: Set(Uuid::now_v7()),
            title: Set(msg.title),
            business_type: Set(msg.bis_type),
            req_method: Set(msg.method),
            uri: Set(msg.uri),
            exec_time: Set(msg.exec_time),
            req_ip: Set(msg.req_ip),
            req_param: Set(msg.req_param),
            success: Set(msg.success),
            err_msg: Set(msg.error_msg),
            create_time: Set(msg.create_time),
            oper_user: Set(msg.oper_user),
        };
        SysOperLogDao::new(&self.state.db).insert(log).await?;
        Ok(())
    }
}
