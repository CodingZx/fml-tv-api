use crate::common::util::time_util::TimeUtil;
use crate::database::model::collect_type::CollectTypeModel;
use crate::database::model::collect_vod::CollectVodModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询采集视频列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectVodListReq {
    pub page: u64,
    pub size: u64,

    pub site_id: Uuid,
    pub name: Option<String>,
    pub type_id: Option<String>,
}


/// 查询采集视频列表-返回
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectVodListResp {
    pub id: Uuid,
    pub site_id: Uuid,
    pub collect_type: String,
    pub vod_id: String,
    pub vod_name: String,
    pub vod_pic: String,
    pub vod_blurb: String,
    pub create_time: String,
    pub episode_count: i32,
}

impl CollectVodListResp {
    pub fn new(model: CollectVodModel, collect_type: Option<CollectTypeModel>) -> Self {
        Self {
            id: model.id,
            site_id: model.site_id,
            collect_type: collect_type.map(|it|it.type_name).unwrap_or_default(),
            vod_id: model.vod_id,
            vod_name: model.vod_name,
            vod_pic: model.vod_pic,
            vod_blurb: model.vod_blurb,
            create_time: TimeUtil::format_default(model.create_time),
            episode_count: model.episode_count,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectEpisodeReq {
    pub site_id: Uuid,
    pub vod_id: Uuid,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectLineResp {
    pub line: String,
    pub player: String,
    pub episodes: Vec<CollectEpisodeResp>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectEpisodeResp {
    pub name: String,
    pub url: String,
}
