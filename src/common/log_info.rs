use crate::common::pgq::service::MessageSender;
use crate::common::state::AppState;
use crate::common::{consts, logger};
use crate::consumer::oper_log_consumer::OperLogMessage;
use crate::database::model::BusinessTypes;
use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BisType {
    OTHER(&'static str),
    SAVE(&'static str),
    SELECT(&'static str),
    DELETE(&'static str),
}

impl BisType {
    pub fn to_str(&self) -> String {
        match self {
            BisType::OTHER(str) => str.to_string(),
            BisType::SAVE(str) => str.to_string(),
            BisType::SELECT(str) => str.to_string(),
            BisType::DELETE(str) => str.to_string(),
        }
    }
}

pub struct OperLogInfo {
    pub title: String,
    pub bis_type: BisType,
    pub method: String,
    pub uri: String,
    pub exec_time: i64,
    pub req_ip: String,
    pub req_param: String,
    pub oper_user: Option<Uuid>,
    pub success: bool,
    pub error_msg: String,
}

pub async fn process_log(state: &AppState, info: OperLogInfo) {
    let bis = match info.bis_type {
        BisType::OTHER(_) => BusinessTypes::Other,
        BisType::SAVE(_) => BusinessTypes::Save,
        BisType::SELECT(_) => BusinessTypes::Select,
        BisType::DELETE(_) => BusinessTypes::Delete,
    };

    let msg = OperLogMessage {
        title: info.title,
        bis_type: bis,
        method: info.method,
        uri: info.uri,
        exec_time: info.exec_time,
        req_ip: info.req_ip,
        req_param: info.req_param,
        oper_user: info.oper_user,
        success: info.success,
        error_msg: info.error_msg,
        create_time: Local::now().naive_local(),
    };

    let sender = MessageSender::new(&state.db);
    if let Err(e) = sender.send_json(consts::queues::OPER_LOG_QUEUE, msg).await
    {
        logger::error!("Oper Log send Error: {}", e);
    }
}