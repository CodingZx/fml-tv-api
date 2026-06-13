use crate::common::util::time_util::TimeUtil;
use crate::database::model::collect_site::{CollectSiteModel, CollectStatus};
use crate::router::admin::vo::VersionIdReq;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteListReq {
    pub page: u64,
    pub size: u64,

    pub name: Option<String>,
    pub main_page: Option<String>,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteListResp {
    pub id: Uuid,
    pub name: String,
    pub main_page: String,
    pub req_url: String,
    pub full_status: i32,
    pub collect_status: i32,
    pub player: String,
    pub last_time: String,
    pub status: bool,
    pub update_time: String,
    pub version: i32,
}

impl CollectSiteListResp {
    pub fn new(model: CollectSiteModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            main_page: model.main_page,
            req_url: model.req_url,
            full_status: model.full_status.get_value(),
            collect_status: model.collect_status.get_value(),
            last_time: if model.full_status == CollectStatus::Completed {
                TimeUtil::format_default(model.last_time)
            } else {
                "".to_string()
            },
            status: model.status,
            update_time: TimeUtil::format_default(model.update_time),
            version: model.version,
            player: model.player,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteSaveReq {
    pub id: Option<Uuid>,
    pub name: String,
    pub main_page: String,
    pub req_url: String,
    pub version: i32,
    pub player: String,
}

/// 发起全量采集
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteFullCollectReq {
    pub id: Uuid,
    pub version: i32,
}

/// 发起增量采集
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteCollectReq {
    pub id: Uuid,
    pub hour: i64,
    pub version: i32,
}

/// 修改状态-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteStatusReq {
    pub id: Uuid,
    pub status: bool,
    pub version: i32,
}

/// 删除信息-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectSiteDeleteReq {
    pub id_ver: Vec<VersionIdReq>,
    pub reason: String,
}

