use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectTypeResp {
    pub id: Uuid,
    pub name: String,
    pub bind_id: Vec<String>,
    pub bind_type: Vec<String>,
    pub vod_count: i32,
    pub show: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectTypeSaveBindReq {
    pub site_id: Uuid,
    pub bind: Vec<CollectTypeBindReq>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectTypeBindReq {
    pub collect_type_id: Uuid,
    pub tv_type_id: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectTypeStatusReq {
    pub id: Vec<Uuid>,
    pub show: bool,
}
