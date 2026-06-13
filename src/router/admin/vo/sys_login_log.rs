use crate::common::util::time_util::TimeUtil;
use crate::database::model::sys_login_log::SysLoginLogModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysLoginLogListReq {
    pub page: u64,
    pub size: u64,

    pub ip: Option<String>,
    pub user_name: Option<String>,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysLoginLogListResp {
    pub id: Uuid,          // 记录ID
    pub ip: String,
    pub user_name: String, // 用户名
    pub password: String, // 密码
    pub create_time: String, // 创建时间
}

impl SysLoginLogListResp {
    pub fn new(log: SysLoginLogModel) -> Self {
        Self {
            id: log.id,
            ip: log.ip_addr,
            user_name: log.user_name,
            password: log.password,
            create_time: TimeUtil::format_default(log.create_time),
        }
    }
}
