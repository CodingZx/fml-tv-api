use crate::common::util::time_util::TimeUtil;
use crate::database::model::tv_vod::TvVodModel;
use crate::database::model::tv_vod_pic::TvVodPicModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvVodListReq {
    pub page: u64,
    pub size: u64,

    pub name: Option<String>,
    pub type_id: Option<String>,
    pub show_status: String,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TvVodListResp {
    pub id: Uuid,            // 记录ID
    pub name: String,        // 名称
    pub type_name: String,
    pub vod_pic: String,
    pub create_time: String,
    pub update_time: String,
    pub episode_count: i32,
    pub show: bool,
    pub collect_sites: Vec<String>,
}

impl TvVodListResp {
    pub fn new(model: TvVodModel, type_name: Vec<String>, sites: Vec<String>, pic: Option<TvVodPicModel>) -> Self {
        Self {
            id: model.id,
            name: model.name,
            type_name: type_name.join(", "),
            vod_pic: pic.map(|t|t.pic).unwrap_or_default(),
            create_time: TimeUtil::format_default(model.create_time),
            update_time: TimeUtil::format_default(model.update_time),
            episode_count: model.episode_count,
            show: model.show,
            collect_sites: sites,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvVodLineResp {
    pub line: String,
    pub player: String,
    pub episodes: Vec<TvVodEpisodeResp>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvVodEpisodeResp {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvVodShowReq {
    pub id: Uuid,
    pub show: bool,
}