use crate::common::util::time_util::TimeUtil;
use crate::database::model::sys_oper_log::SysOperLogModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogListReq {
    pub page: u64,
    pub size: u64,

    pub start: Option<String>,
    pub end: Option<String>,
}

/// 列表数据-返回
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogListResp {
    pub id: Uuid,
    pub title: String,
    pub uri: String,
    pub req_method: String,
    pub exec_time: i64,
    pub req_ip: String,
    pub oper_name: String,
    pub success: bool,
    pub create_time: String,
}

impl SysOperLogListResp {
    pub fn new(model: SysOperLogModel, op_name: String) -> Self {
        Self {
            id: model.id,
            title: model.title,
            uri: model.uri,
            req_method: model.req_method,
            exec_time: model.exec_time,
            req_ip: model.req_ip,
            oper_name: op_name,
            success: model.success,
            create_time: TimeUtil::format_default(model.create_time),
        }
    }
}

/// 详情数据-返回
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogDetailResp {
    pub id: Uuid,
    pub title: String,
    pub uri: String,
    pub bis_type: String,
    pub req_method: String,
    pub exec_time: i64,
    pub req_ip: String,
    pub oper_name: String,
    pub req_param: String,
    pub error_msg: String,
    pub create_time: String,
    pub success: bool,
}

impl SysOperLogDetailResp {
    pub fn new(model: SysOperLogModel, op_name: String) -> Self {
        Self {
            id: model.id,
            title: model.title,
            uri: model.uri,
            bis_type: model.business_type.get_name(),
            req_method: model.req_method,
            exec_time: model.exec_time,
            req_ip: model.req_ip,
            oper_name: op_name,
            req_param: model.req_param,
            error_msg: model.err_msg,
            create_time: TimeUtil::format_default(model.create_time),
            success: model.success,
        }
    }
}
