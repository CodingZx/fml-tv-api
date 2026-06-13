use crate::common::model::Page;
use crate::common::pgq::consumer::MessageConsumer;
use crate::common::state::AppState;
use crate::common::{consts, ServerResult};
use crate::database::dao::collect_type::CollectTypeDao;
use crate::database::dao::tv_type_bind::TvTypeBindDao;
use crate::database::dao::tv_vod::TvVodDao;
use crate::database::model::collect_type::CollectTypeModel;
use crate::database::model::tv_type_bind::TvTypeBindModel;
use crate::database::model::tv_vod::{TvVodActiveModel, TvVodModel};
use crate::database::model::HashUUIDs;
use async_trait::async_trait;
use sea_orm::{Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TvVodRebuildMessage {
    pub collect_type_id: Uuid
}

pub struct TvVodRebuildConsumer {
    state: Arc<AppState>,
}

impl TvVodRebuildConsumer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    async fn build_status(&self, vod: &TvVodModel, collect_type_cache: &mut HashMap<Uuid, CollectTypeModel>) -> ServerResult<BuildStatus> {
        let mut show_status = true;

        let collect_type_dao = CollectTypeDao::new(&self.state.db);
        for collect_type_id in vod.collect_type.0.iter() {
            if show_status {
                let show = match collect_type_cache.get(&collect_type_id) {
                    None => {
                        let collect_type_op = collect_type_dao.find_by_id(*collect_type_id).await?;
                        let status = if let Some(v) = collect_type_op {
                            let status = v.show;
                            collect_type_cache.insert(*collect_type_id, v);
                            status
                        } else {
                            true
                        };
                        status
                    },
                    Some(v) => v.show,
                };
                if !show {
                    // 如果有任何不显示的, 则整体设置为不显示
                    show_status = false;
                }
            }
        }
        Ok(BuildStatus {
            show: show_status,
        })
    }

    async fn build_tv_types(&self, vod: &TvVodModel, type_bind_cache: &mut HashMap<Uuid, Vec<TvTypeBindModel>>) -> ServerResult<HashSet<Uuid>> {
        let mut tv_types = HashSet::new();

        let tv_type_bind_dao = TvTypeBindDao::new(&self.state.db);
        for collect_type_id in vod.collect_type.0.iter() {
            match type_bind_cache.get(&collect_type_id) {
                None => {
                    let binds = tv_type_bind_dao.find_by_collect_type(*collect_type_id).await?;
                    for bind in binds.iter() {
                        tv_types.insert(bind.tv_type_id);
                    }
                    type_bind_cache.insert(*collect_type_id, binds);
                },
                Some(v) => {
                    for bind in v.iter() {
                        tv_types.insert(bind.tv_type_id);
                    }
                }
            }
        }
        Ok(tv_types)
    }
}

#[async_trait]
impl MessageConsumer for TvVodRebuildConsumer {
    fn queue(&self) -> String {
        consts::queues::TV_VOD_REBUILD_QUEUE.to_string()
    }

    fn count(&self) -> u32 {
        1
    }

    async fn consume(&self, msg: &str) -> ServerResult<()> {
        let msg: TvVodRebuildMessage = serde_json::from_str(msg)?;
        let mut page = 1;
        let size = 1000;

        let mut collect_type_cache: HashMap<Uuid, CollectTypeModel> = HashMap::new();
        let mut tv_type_bind_cache: HashMap<Uuid, Vec<TvTypeBindModel>> = HashMap::new();

        loop {
            let vod_list = TvVodDao::new(&self.state.db).find_page_by_collect_type_id(Page::from(page, size), msg.collect_type_id).await?;
            if vod_list.is_empty() { break }

            let mut updates: Vec<TvVodActiveModel> = Vec::new();
            for vod in vod_list {
                let mut update_model = TvVodActiveModel {
                    id: Set(vod.id),
                    ..Default::default()
                };
                let mut need_update = false;
                let build_status = self.build_status(&vod, &mut collect_type_cache).await?;
                if vod.show != build_status.show {
                    need_update = true;
                    update_model.show = Set(build_status.show);
                }

                let tv_types = self.build_tv_types(&vod, &mut tv_type_bind_cache).await?;
                if !vod.tv_type.0.eq(&tv_types) {
                    need_update = true;
                    update_model.tv_type = Set(HashUUIDs(tv_types));
                }

                if need_update {
                    updates.push(update_model);
                }
            }
            // 修改
            if !updates.is_empty() {
                self.state.db.transaction(|tx| {
                    Box::pin(async move {
                        let dao = TvVodDao::new(tx);
                        for update_model in updates {
                            dao.update_by_id(update_model).await?;
                        }
                        Ok(())
                    })
                }).await?;
            }
            page = page + 1;
        }
        Ok(())
    }
}

struct BuildStatus {
    show: bool,
}