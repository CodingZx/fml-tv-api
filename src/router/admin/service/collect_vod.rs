use crate::common::error::ServerError::BusinessError;
use crate::common::model::{Page, Pager};
use crate::common::state::AppState;
use crate::common::ServerResult;
use crate::database::dao::collect_site::CollectSiteDao;
use crate::database::dao::collect_type::CollectTypeDao;
use crate::database::dao::collect_vod::CollectVodDao;
use crate::database::dao::collect_vod_episode::CollectVodEpiDao;
use crate::database::model::collect_vod::CollectVodColumn;
use crate::router::admin::vo::collect_vod::{CollectEpisodeReq, CollectEpisodeResp, CollectLineResp, CollectVodListReq, CollectVodListResp};
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct CollectVodService {
    state: Arc<AppState>,
}

impl CollectVodService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn list(&self, param: CollectVodListReq) -> ServerResult<Pager<CollectVodListResp>> {
        let mut condition = Condition::all();
        condition = condition.add(CollectVodColumn::SiteId.eq(param.site_id));
        if let Some(id) = param.type_id {
            if let Ok(type_id) = Uuid::from_str(&id) {
                condition = condition.add(CollectVodColumn::CollectTypeId.eq(type_id));
            }
        }
        let name = param.name.unwrap_or_default();
        if !name.is_empty() {
            condition = condition.add(CollectVodColumn::VodName.contains(&name));
        }
        let page = Page::from(param.page, param.size);
        let (record, total) = CollectVodDao::new(&self.state.db).find_list(page, condition).await?;

        let mut resp = Vec::with_capacity(record.len());
        for rec in record {
            let typ = CollectTypeDao::new(&self.state.db).find_by_id(rec.collect_type_id).await?;
            resp.push(CollectVodListResp::new(rec, typ));
        }
        Ok(Pager::new(resp, total))
    }

    pub async fn episode(&self, param: CollectEpisodeReq) -> ServerResult<Vec<CollectLineResp>> {
        let site = CollectSiteDao::new(&self.state.db).find_by_id(param.site_id).await?.ok_or(BusinessError("站点不存在"))?;
        let episodes = CollectVodEpiDao::new(&self.state.db).find_by_vod_id(param.vod_id).await?;

        let mut lines = HashMap::<String, Vec<CollectEpisodeResp>>::new();
        for episode in episodes {
            let line = episode.line;
            let resp = CollectEpisodeResp {
                name: episode.name,
                url: episode.url,
            };
            lines.entry(line).or_insert(vec![]).push(resp);
        }
        let mut resp = Vec::with_capacity(lines.len());
        for (line, episodes) in lines {
            let player = site.player.clone();
            resp.push(CollectLineResp { line, player, episodes })
        }
        Ok(resp)

    }
}