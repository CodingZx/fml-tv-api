use crate::common::error::ServerError::BusinessError;
use crate::common::model::{IdReq, Page, Pager};
use crate::common::state::AppState;
use crate::common::ServerResult;
use crate::database::dao::collect_site::CollectSiteDao;
use crate::database::dao::collect_vod::CollectVodDao;
use crate::database::dao::collect_vod_episode::CollectVodEpiDao;
use crate::database::dao::tv_type::TvTypeDao;
use crate::database::dao::tv_type_bind::TvTypeBindDao;
use crate::database::dao::tv_vod::TvVodDao;
use crate::database::dao::tv_vod_pic::TvVodPicDao;
use crate::database::model::tv_vod::TvVodColumn;
use crate::router::admin::vo::tv_vod::{TvVodEpisodeResp, TvVodLineResp, TvVodListReq, TvVodListResp, TvVodShowReq};
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, Condition};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
use crate::database::dao::collect_type::CollectTypeDao;

pub struct TvVodService {
    state: Arc<AppState>,
}

impl TvVodService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// 获得列表
    pub async fn list(&self, param: TvVodListReq) -> ServerResult<Pager<TvVodListResp>> {
        let name = param.name.unwrap_or_default();
        let type_id = match Uuid::from_str(&param.type_id.unwrap_or_default()) {
            Ok(id) => Some(id),
            Err(_) => None,
        };
        let mut conditions = Condition::all();
        if !name.is_empty() {
            conditions = conditions.add(TvVodColumn::Name.contains(name));
        }
        let show_status = match bool::from_str(&param.show_status) {
            Ok(v) => Some(v),
            Err(_) => None,
        };
        if let Some(v) = show_status {
            conditions = conditions.add(TvVodColumn::Show.eq(v));
        }

        let tv_type_bind_dao = TvTypeBindDao::new(&self.state.db);
        if let Some(v) = type_id {
            let array_literal = format!(r#" "tv_type" ?| ARRAY['{v}'] "#);
            conditions = conditions.add(Expr::cust(array_literal));
        }

        let page = Page::from(param.page, param.size);
        let (records, total) = TvVodDao::new(&self.state.db).find_page_list(page, conditions).await?;

        let collect_type_dao = CollectTypeDao::new(&self.state.db);
        let collect_site_dao = CollectSiteDao::new(&self.state.db);
        let tv_type_dao = TvTypeDao::new(&self.state.db);
        let vod_pic_dao = TvVodPicDao::new(&self.state.db);
        let mut result = Vec::new();
        for record in records {
            let collect_type_ids = record.collect_type.0.clone();
            let collect_types = collect_type_dao.find_by_ids(collect_type_ids.clone()).await?;
            let site_ids = collect_types.iter().map(|t| t.site_id).collect::<HashSet<_>>();

            let collect_type_ids = record.collect_type.0.clone();
            let tv_type_ids = tv_type_bind_dao.find_by_collect_types(collect_type_ids).await?.iter().map(|t|t.tv_type_id).collect::<HashSet<_>>();

            let pic = vod_pic_dao.find_one_by_vod_id(record.id).await?;
            let types = tv_type_dao.find_by_ids(tv_type_ids).await?.into_iter().map(|t|t.name).collect::<Vec<_>>();
            let sites = collect_site_dao.find_by_ids(site_ids).await?.into_iter().map(|t| {
                let collect_types = collect_types.iter().filter(|c| c.site_id == t.id)
                    .map(|c| c.type_name.clone())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}-{}", t.name, collect_types)
            }).collect::<Vec<_>>();
            result.push(TvVodListResp::new(record, types, sites, pic));
        }
        Ok(Pager::new(result, total))
    }

    pub async fn episode(&self, param: IdReq) -> ServerResult<Vec<TvVodLineResp>> {
        let vod = TvVodDao::new(&self.state.db).find_by_id(param.id).await?.ok_or(BusinessError("ID错误, 数据不存在"))?;

        let collect_vod_dao = CollectVodDao::new(&self.state.db);
        let collect_site_dao = CollectSiteDao::new(&self.state.db);
        let collect_epi_dao = CollectVodEpiDao::new(&self.state.db);
        let mut resp = Vec::new();
        let mut unknow_site_idx = 0;
        for collect_vod_id in vod.collect_vod.0 {
            let site_id = collect_vod_dao.find_site_by_id(collect_vod_id).await?;
            let site = collect_site_dao.find_by_id(site_id).await?;
            let episodes = collect_epi_dao.find_by_vod_id(collect_vod_id).await?;

            let (site_name, player) = if let Some(v) = site {
                (v.name, v.player)
            } else {
                unknow_site_idx = unknow_site_idx + 1;
                (format!("未知站点{unknow_site_idx}"), "".to_string())
            };

            let mut lines = HashMap::<String, Vec<TvVodEpisodeResp>>::new();
            for episode in episodes {
                let line = episode.line;
                let resp = TvVodEpisodeResp {
                    name: episode.name,
                    url: episode.url,
                };
                lines.entry(line).or_insert(vec![]).push(resp);
            }

            for (line_name, episodes) in lines {
                let line = format!("{site_name}-{line_name}");
                let player = player.clone();
                resp.push(TvVodLineResp { line, player, episodes })
            }
        }
        Ok(resp)
    }

    pub async fn update_status(&self, param: TvVodShowReq) -> ServerResult<()> {
        TvVodDao::new(&self.state.db).update_show(param.id, param.show).await?;
        Ok(())
    }

}
