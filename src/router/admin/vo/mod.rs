use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod collect_site;
pub mod collect_type;
pub mod collect_vod;
pub mod login;
pub mod sys_account;
pub mod sys_config;
pub mod sys_login_log;
pub mod sys_oper_log;
pub mod sys_queue_msg;
pub mod tv_group;
pub mod tv_type;
pub mod tv_vod;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComBoxResp {
    pub id: String,
    pub name: String,
}

impl ComBoxResp {
    pub fn from<T: ToString, F: ToString>(id: T, name: F) -> Self {
        let id = id.to_string();
        let name = name.to_string();
        Self { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionIdReq {
    pub id: Uuid,
    pub version: i32,
}
