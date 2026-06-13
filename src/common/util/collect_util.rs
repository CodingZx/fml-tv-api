use crate::common;
use crate::common::error::ServerError::BusinessStrError;
use crate::common::ServerResult;
use serde::{Deserialize, Serialize};
use std::time::Duration;
/*
视频（Vod）
- 路径：/router.php/provide/vod/
- 主要参数：
  - ac：list|detail|videolist（默认 list）
  - at：json|xml（默认 json）
  - t：分类ID
  - ids：ID 列表，逗号分隔
  - pg：页码，默认 1
  - pagesize：每页条数，最大 100
  - wd：搜索关键字
  - h：最近 N 小时内（如 24）
  - year：年份（如 2020 或 2018-2022）
  - isend：完结筛选（1 完结，0 连载）
  - from：播放器筛选（逗号分隔），影响返回的 vod_play_from/url 等字段
  - sort_direction：desc|asc（默认 desc），影响按 time 排序方向
- 备注：
  - ac=list 时返回精简字段并附带分类 class；ac=videolist/detail 返回完整字段（含播放组等）。
  - at=xml 时返回 V1 文档所示 XML 结构；json 返回字段见源码处理（vod_json/vod_xml）。
*/

/// Mac cms 10 List请求 返回
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CollectListResult {
    #[serde(deserialize_with = "common::i32_from_any")]
    pub code: i32,
    #[serde(deserialize_with = "common::str_from_any")]
    pub msg: String,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub page: i32,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub pagecount: i32,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub limit: i32,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub total: i32,
    // 其他忽略
}

/// Mac cms 10 detail/videolist请求 返回
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CollectVideoListResult {
    #[serde(deserialize_with = "common::i32_from_any")]
    pub code: i32,
    #[serde(deserialize_with = "common::str_from_any")]
    pub msg: String,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub page: i32,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub pagecount: i32,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub limit: i32,
    #[serde(deserialize_with = "common::i32_from_any")]
    pub total: i32,
    pub list: Vec<serde_json::Value>,
}

impl CollectVideoListResult {

    fn to_vod_list(self) -> ServerResult<CollectVodList> {
        let mut vod_list = Vec::with_capacity(self.list.len());
        for val in self.list {
            let vod = serde_json::from_value(val.clone())?;
            vod_list.push(vod)
        }
        let r = CollectVodList {
            code: self.code,
            msg: self.msg,
            page: self.page,
            pagecount: self.pagecount,
            limit: self.limit,
            total: self.total,
            list: vod_list,
        };
        Ok(r)
    }
}

#[derive(Debug, Clone)]
pub struct CollectVodList {
    pub code: i32,
    pub msg: String,
    pub page: i32,
    pub pagecount: i32,
    pub limit: i32,
    pub total: i32,
    pub list: Vec<CollectVod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectVod {
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_id: Option<String>, // 视频ID
    #[serde(deserialize_with = "common::i32_from_any")]
    pub type_id: i32, // 类型ID
    #[serde(deserialize_with = "common::i32_from_any")]
    pub type_id_1: i32, // 类型的上级ID
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub type_name: Option<String>, // 分类名称
    pub vod_name: String, // 视频名称
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_sub: Option<String>, // 缩略名称
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_tag: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_class: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_actor: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_remarks: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_pic: Option<String>, // 视频图片
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_time: Option<String>, //  2026-05-24 12:57:50
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_blurb: Option<String>, // 介绍
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_content: Option<String>, // 视频内容
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_play_from: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_play_server: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_play_note: Option<String>,
    #[serde(deserialize_with = "common::option_str_from_any")]
    pub vod_play_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maccms10VodEpisode {
    pub name: String,
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maccms10VodLine {
    pub line: String,
    pub episode: Vec<Maccms10VodEpisode>,
}

impl CollectVod {
    pub fn parse_vod_episode_list(vod_play_note: &str, vod_play_url: &str) -> ServerResult<Vec<Maccms10VodLine>> {
        let play_note = vod_play_note;
        let play_url = vod_play_url;

        let mut lines = Vec::new();
        if play_note.is_empty() {
            // 只有一条线
            let episodes = get_play_urls(&play_url);
            let line = Maccms10VodLine {
                line: "线路1".to_string(),
                episode: episodes,
            };
            return Ok(vec![line])
        } else {
            // 多条
            let mut line = 1;
            let urls = play_url.split(&play_note).collect::<Vec<_>>();
            for url in urls {
                let line_str = format!("线路{}", line);
                let episodes = get_play_urls(url);
                lines.push(Maccms10VodLine {
                    line: line_str,
                    episode: episodes,
                });
                line = line + 1;
            }
        }
        Ok(lines)
    }
}

fn get_play_urls(play_url: &str) -> Vec<Maccms10VodEpisode> {
    let mut episode = Vec::new();
    let split = play_url.split('#').collect::<Vec<_>>();
    for play_url in split {
        let e = play_url.split('$').collect::<Vec<_>>();
        if e.len() < 2 {
            continue; // 无效的集数
        }
        episode.push(Maccms10VodEpisode {
            name: e[0].to_string(),
            url: e[1].to_string(),
        })
    }
    episode
}

/// 请求获取List接口
pub async fn fetch_list(url: &str) -> ServerResult<()> {
    let request_url = format!("{url}?ac=list&at=json");
    let response = request(&request_url).await?;
    let _ = serde_json::from_str::<CollectListResult>(&response)?;
    Ok(())
}

/// 请求获取VideoList接口
pub async fn fetch_video_list(url: &str, page: i32, size: i32, hour: Option<i64>) -> ServerResult<(String, CollectVodList)> {
    let hour = match hour {
        Some(hour) => hour.to_string(),
        None => "".to_string(),
    };
    let request_url = format!("{url}?ac=videolist&at=json&pg={page}&pagesize={size}&h={hour}&wd=&sort_direction=asc");
    let response = request(&request_url).await?;
    let result = serde_json::from_str::<CollectVideoListResult>(&response)?;
    Ok((response, result.to_vod_list()?))
}

const UA:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36";

/// 发起请求
async fn request(url: &str) -> ServerResult<String> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .read_timeout(Duration::from_secs(30))
        .build()?;
    let response = client.get(url).header("user-agent", UA).send().await?;
    if !response.status().is_success() {
        return Err(BusinessStrError(format!("请求接口失败, 返回Status为[{}]", response.status())));
    }
    Ok(response.text().await?)
}
