use crate::database::model::tv_type::TvTypeModel;
use crate::database::model::tv_vod::TvVodModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JsQuery {
    pub types: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeGroupReq {
    pub req_token: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeListResp {
    pub id: Uuid,
    pub name: String,
}

impl TypeListResp {
    pub fn new(model: TvTypeModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodListReq {
    pub req_token: String,
    pub page: u64,
    pub size: u64,
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodListResp {
    pub id: Uuid,
    pub name: String,
    pub cover: String,
    pub episodes: i32,
}

impl VodListResp {

    pub fn from(model: &TvVodModel, cover: String) -> Self {
        Self {
            id: model.id,
            name: model.name.clone(),
            cover,
            episodes: model.episode_count,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodSearchReq {
    pub req_token: String,
    pub page: u64,
    pub size: u64,
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodDetailReq {
    pub req_token: String,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodDetailResp {
    pub id: Uuid,
    pub name: String,
    pub cover: String,
    pub genre: String,
    pub intro: String,
    pub description: String,
    pub lines: Vec<VodEpisodeLineResp>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodEpisodePlayReq {
    pub req_token: String,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodEpisodePlayResp {
    pub url: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodEpisodeLineResp {
    pub line: String,
    pub episodes: Vec<VodEpisodeResp>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VodEpisodeResp {
    pub id: Uuid,
    pub name: String,
    pub order: i32,
}


