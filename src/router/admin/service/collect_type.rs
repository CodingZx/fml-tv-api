use crate::common::model::IdReq;
use crate::common::pgq::service::MessageSender;
use crate::common::state::AppState;
use crate::common::{consts, ServerResult};
use crate::consumer::tv_vod_rebuild_consumer::TvVodRebuildMessage;
use crate::database::dao::collect_type::CollectTypeDao;
use crate::database::dao::tv_type::TvTypeDao;
use crate::database::dao::tv_type_bind::TvTypeBindDao;
use crate::database::model::tv_type_bind::TvTypeBindActiveModel;
use crate::router::admin::vo::collect_type::{CollectTypeResp, CollectTypeSaveBindReq, CollectTypeStatusReq};
use crate::router::admin::vo::ComBoxResp;
use sea_orm::{Set, TransactionTrait};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct CollectTypeService {
    state: Arc<AppState>,
}

impl CollectTypeService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn all(&self, param: IdReq) -> ServerResult<Vec<ComBoxResp>> {
        let types = CollectTypeDao::new(&self.state.db)
            .find_by_site_id(param.id)
            .await?
            .into_iter()
            .map(|t| ComBoxResp::from(t.id, t.type_name))
            .collect::<Vec<_>>();
        Ok(types)
    }

    pub async fn site_types(&self, param: IdReq) -> ServerResult<Vec<CollectTypeResp>> {
        let collect_types = CollectTypeDao::new(&self.state.db).find_by_site_id(param.id).await?;
        let collect_type_ids = collect_types.iter().map(|it| it.id).collect::<Vec<_>>();
        let binds = TvTypeBindDao::new(&self.state.db).find_by_collect_types(collect_type_ids).await?;

        let tv_types = TvTypeDao::new(&self.state.db).find_all().await?.into_iter().map(|it|(it.id, it)).collect::<HashMap<_, _>>();
        let mut result = Vec::new();
        for typ in collect_types {
            let mut resp = CollectTypeResp {
                id: typ.id,
                name: typ.type_name,
                bind_id: Vec::new(),
                bind_type: Vec::new(),
                vod_count: typ.vod_count,
                show: typ.show,
            };
            let binds = binds.iter().filter(|&it| it.collect_type_id == typ.id).collect::<Vec<_>>();
            for bind in binds {
                if let Some(tv_type) = tv_types.get(&bind.tv_type_id) {
                    resp.bind_id.push(tv_type.id.to_string());
                    resp.bind_type.push(tv_type.name.clone());
                }
            }
            result.push(resp);
        }
        Ok(result)
    }

    pub async fn bind_types(&self, param: CollectTypeSaveBindReq) -> ServerResult<()> {
        let mut bind_vec = Vec::new();
        let mut rebuild_vec = Vec::new();
        let collect_type_dao = CollectTypeDao::new(&self.state.db);

        let mut old_bind_maps: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();
        {
            let old_binds = TvTypeBindDao::new(&self.state.db).find_by_site_id(param.site_id).await?;
            for old_bind in old_binds {
                let key = old_bind.collect_type_id;
                if let Some(v) = old_bind_maps.get_mut(&key) {
                    v.insert(old_bind.tv_type_id);
                } else {
                    let mut binds = HashSet::new();
                    binds.insert(old_bind.tv_type_id);
                    old_bind_maps.insert(key, binds);
                }
            }
        }
        
        for bind in param.bind {
            let mut bind_type_ids = HashSet::new();
            {
                for tv_type_id in bind.tv_type_id {
                    if "not" == tv_type_id {
                        bind_type_ids.clear();
                        break
                    }
                    let bind_type_id = match Uuid::from_str(&tv_type_id) {
                        Ok(id) => id,
                        Err(_) => continue,
                    };
                    bind_type_ids.insert(bind_type_id);
                }
            }

            let mut rebuild = false;
            {
                // 之前有绑定的
                if let Some(old_bind_id) = old_bind_maps.get(&bind.collect_type_id) {
                    if !old_bind_id.eq(&bind_type_ids) {
                        // 绑定的ID变了
                        rebuild = true;
                    }
                    if bind_type_ids.is_empty(){
                        // 之前绑定过, 但是现在取消绑定了
                        rebuild = true;
                    }
                } else if bind_type_ids.len() > 0 {
                    // 之前没绑定, 但是现在绑定了
                    rebuild = true;
                }
            }

            if rebuild {
                rebuild_vec.push(TvVodRebuildMessage {
                    collect_type_id: bind.collect_type_id,
                });
            }
            
            if bind_type_ids.is_empty() {
                continue;
            }

            let tv_types = TvTypeDao::new(&self.state.db).find_by_ids(bind_type_ids).await?;
            for tv_type in tv_types {
                if let Some(collect_type) = collect_type_dao.find_by_id(bind.collect_type_id).await? {
                    if collect_type.site_id != param.site_id {
                        continue;
                    }
                    bind_vec.push(TvTypeBindActiveModel {
                        id: Set(Uuid::now_v7()),
                        tv_type_id: Set(tv_type.id),
                        collect_type_id: Set(collect_type.id),
                        site_id: Set(param.site_id),
                    });
                }
            }
        }
        if !bind_vec.is_empty() {
            self.state.db.transaction(|tx| {
                Box::pin(async move {
                    let tv_type_bind_dao = TvTypeBindDao::new(tx);
                    tv_type_bind_dao.delete_by_site_id(param.site_id).await?;
                    tv_type_bind_dao.batch_insert(bind_vec).await?;

                    let msg_sender = MessageSender::new(tx);
                    for msg in rebuild_vec {
                        msg_sender.send_json(consts::queues::TV_VOD_REBUILD_QUEUE, msg).await?;
                    }
                    Ok(())
                })
            }).await?;
        }
        Ok(())
    }

    pub async fn update_status(&self, param: CollectTypeStatusReq) -> ServerResult<()> {
        self.state.db.transaction(|tx| {
            Box::pin(async move {
                for id in param.id {
                    CollectTypeDao::new(tx).update_show(id, param.show).await?;
                    let msg = TvVodRebuildMessage {
                        collect_type_id: id,
                    };
                    MessageSender::new(tx).send_delay_json_1s(consts::queues::TV_VOD_REBUILD_QUEUE, msg).await?;
                }
                Ok(())
            })
        }).await?;
        Ok(())
    }

}