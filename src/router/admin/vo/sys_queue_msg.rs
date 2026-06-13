use crate::common;
use crate::common::pgq::entity::message::SysQueueMsgModel;
use crate::common::pgq::MessageStatus;
use crate::common::util::time_util::TimeUtil;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysQueueMsgListReq {
    pub page: u64,
    pub size: u64,

    pub queue: Option<String>,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub status: i32,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysQueueMsgListResp {
    pub id: Uuid,
    pub queue: String,
    pub message: String,
    pub status: i32,
    pub error: String,
    pub finish_time: String,
    pub publish_time: String,
    pub process_time: String,
}

impl SysQueueMsgListResp {
    pub fn new(model: SysQueueMsgModel) -> Self {
        Self {
            id: model.id,
            queue: model.queue,
            message: model.message,
            status: model.status.value(),
            error: model.error_detail,
            publish_time: TimeUtil::format_default(model.publish_time),
            process_time: if model.status != MessageStatus::Pending {
                TimeUtil::format_default(model.process_time)
            } else {
                "".to_string()
            },
            finish_time: if model.status == MessageStatus::Success || model.status == MessageStatus::Failed {
                TimeUtil::format_default(model.finish_time)
            } else {
                "".to_string()
            },
        }
    }
}
