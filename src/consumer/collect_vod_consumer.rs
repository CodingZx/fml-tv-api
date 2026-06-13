use crate::common::error::ServerError::BusinessError;
use crate::common::pgq::consumer::MessageConsumer;
use crate::common::state::AppState;
use crate::common::util::collect_util::CollectVod;
use crate::common::util::time_util::TimeUtil;
use crate::common::{consts, ServerResult};
use crate::database::dao::collect_site::CollectSiteDao;
use crate::database::dao::collect_type::CollectTypeDao;
use crate::database::dao::collect_vod::CollectVodDao;
use crate::database::dao::collect_vod_episode::CollectVodEpiDao;
use crate::database::dao::tv_type_bind::TvTypeBindDao;
use crate::database::dao::tv_vod::TvVodDao;
use crate::database::dao::tv_vod_pic::TvVodPicDao;
use crate::database::model::collect_site::CollectSiteModel;
use crate::database::model::collect_type::CollectTypeActiveModel;
use crate::database::model::collect_vod::CollectVodActiveModel;
use crate::database::model::collect_vod_episode::CollectVodEpiActiveModel;
use crate::database::model::tv_vod::TvVodActiveModel;
use crate::database::model::tv_vod_pic::TvVodPicActiveModel;
use crate::database::model::HashUUIDs;
use async_trait::async_trait;
use chrono::Local;
use sea_orm::{DatabaseTransaction, NotSet, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

/// 视频处理队列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectVodMessage {
    pub site_id: Uuid,
    pub vod: CollectVod,
    pub full: bool,
}

pub(super) struct CollectVodConsumer {
    state: Arc<AppState>,
}

impl CollectVodConsumer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl MessageConsumer for CollectVodConsumer {
    fn queue(&self) -> String {
        consts::queues::COLLECT_VOD_QUEUE.to_string()
    }

    fn count(&self) -> u32 {
        1
    }

    async fn consume(&self, msg: &str) -> ServerResult<()> {
        let msg: CollectVodMessage = serde_json::from_str(msg)?;
        let site = CollectSiteDao::new(&self.state.db).find_by_id(msg.site_id).await?.ok_or_else(||BusinessError("采集站ID错误, 数据不存在"))?;
        self.state.db.transaction(|tx| {
            Box::pin(async move {
                process_vod(tx, site, msg.vod, msg.full).await?;
                Ok(())
            })
        }).await?;
        Ok(())
    }
}

async fn process_vod(tx: &DatabaseTransaction, site: CollectSiteModel, vod: CollectVod, full: bool) -> ServerResult<()> {
    let tv_type_bind_dao = TvTypeBindDao::new(tx);
    let tv_vod_dao = TvVodDao::new(tx);
    let tv_vod_pic_dao = TvVodPicDao::new(tx);
    let collect_type_dao = CollectTypeDao::new(tx);
    let collect_vod_dao = CollectVodDao::new(tx);
    let collect_vod_epi_dao = CollectVodEpiDao::new(tx);
    
    let vod_id = vod.vod_id.unwrap_or_default();
    if vod_id.is_empty() { return Ok(()); }
    let type_name = vod.type_name.unwrap_or_default();
    if type_name.is_empty() { return Ok(()); }
    if vod.vod_name.is_empty() { return Ok(()); }

    // 播放线路和剧集
    let lines = CollectVod::parse_vod_episode_list(&vod.vod_play_note.unwrap_or_default(), &vod.vod_play_url.unwrap_or_default())?;

    // 当前最大集数
    let mut max_episodes = 0i32;
    for line in lines.iter() {
        let line_episodes = line.episode.len() as i32;
        if  line_episodes > max_episodes {
            max_episodes = line_episodes;
        }
    }

    let collect_type_op = collect_type_dao.find_by_type_id(site.id, vod.type_id).await?;
    let collect_type_id = match collect_type_op {
        Some(collect_type) => {
            collect_type.id
        }
        None => {
            let collect_type_id = Uuid::now_v7();
            let new_type = CollectTypeActiveModel {
                id: Set(collect_type_id),
                site_id: Set(site.id),
                type_id: Set(vod.type_id),
                type_name: Set(type_name),
                vod_count: Set(0),
                show: Set(true),
            };
            collect_type_dao.insert(new_type).await?;
            collect_type_id
        }
    };
    let mut pic = vod.vod_pic.unwrap_or_default();
    if pic.len() > 500 {
        pic = String::new();
    }

    let vod_clear_name = vod.vod_name.replace(" ","").to_string();

    let vod_name = vod.vod_name;
    let vod_tag = vod.vod_tag.unwrap_or_default();
    let vod_class = vod.vod_class.unwrap_or_default();
    let vod_actor = vod.vod_actor.unwrap_or_default();
    let vod_blurb = vod.vod_blurb.unwrap_or_default();
    let vod_remarks = vod.vod_remarks.unwrap_or_default();
    let vod_content = vod.vod_content.unwrap_or_default();
    let mut insert_model = CollectVodActiveModel {
        id: NotSet,
        site_id: NotSet,
        collect_type_id: Set(collect_type_id),
        vod_id: Set(vod_id.clone()),
        vod_pic: Set(pic.clone()),
        vod_name: Set(vod_name.clone()),
        vod_tag: Set(vod_tag.clone()),
        vod_class: Set(vod_class.clone()),
        vod_actor: Set(vod_actor.clone()),
        vod_blurb: Set(vod_blurb.clone()),
        vod_remarks: Set(vod_remarks.clone()),
        vod_content: Set(vod_content.clone()),
        create_time: NotSet,
        episode_count: Set(max_episodes),
    };

    let create_time = if full {
        TimeUtil::parse_format(&vod.vod_time.unwrap_or_default(), "%Y-%m-%d %H:%M:%S").unwrap_or(Local::now().naive_local())
    } else {
        Local::now().naive_local()
    };
    let collect_vod = collect_vod_dao.find_by_vod_id(site.id, &vod_id).await?;
    let collect_vod_id = if let Some(model) = collect_vod {
        insert_model.id = Set(model.id);
        collect_vod_dao.update_by_id(insert_model).await?;
        model.id
    } else {
        let collect_new_id = Uuid::now_v7();
        insert_model.id = Set(collect_new_id);
        insert_model.site_id = Set(site.id);
        insert_model.create_time = Set(create_time);
        collect_vod_dao.insert(insert_model).await?;
        collect_type_dao.increment_vod(collect_type_id, 1).await?;
        collect_new_id
    };

    let mut episode_sort = 1;
    let mut episodes = Vec::new();
    for line in lines {
        for episode in line.episode {
            episodes.push(CollectVodEpiActiveModel {
                id: Set(Uuid::now_v7()),
                site_id: Set(site.id),
                collect_vod_id: Set(collect_vod_id),
                line: Set(line.line.clone()),
                name: Set(episode.name),
                url: Set(episode.url),
                sort_num: Set(episode_sort),
            });
            episode_sort = episode_sort + 1;
        }
    }
    collect_vod_epi_dao.delete_by_vod_id(collect_vod_id).await?;
    collect_vod_epi_dao.batch_insert(episodes).await?;

    let tv_vod = tv_vod_dao.find_by_clear_name(&vod_clear_name).await?;
    if let Some(v) = tv_vod {
        let mut update = false;
        let mut update_tv_vod = TvVodActiveModel {
            id: Set(v.id),
            ..Default::default()
        };

        let mut collect_types = v.collect_type.0;
        if collect_types.insert(collect_type_id) {
            update_tv_vod.collect_type = Set(HashUUIDs(collect_types.clone()));
            update = true;
        }

        let mut collect_vod = v.collect_vod.0;
        if collect_vod.insert(collect_vod_id) {
            update_tv_vod.collect_vod = Set(HashUUIDs(collect_vod.clone()));
            update = true;
        }

        let tv_types = tv_type_bind_dao.find_by_collect_types(collect_types).await?
            .iter()
            .map(|t|t.tv_type_id)
            .collect::<HashSet<Uuid>>();

        if !v.tv_type.0.eq(&tv_types) {
            update = true;
            update_tv_vod.tv_type = Set(HashUUIDs(tv_types));
        }

        if v.vod_tag.is_empty() {
            update_tv_vod.vod_tag = Set(vod_tag);
            update = true;
        }
        if v.vod_class.is_empty() {
            update_tv_vod.vod_class = Set(vod_class);
            update = true;
        }
        if v.vod_actor.is_empty() {
            update_tv_vod.vod_actor = Set(vod_actor);
            update = true;
        }
        if v.vod_blurb.is_empty() {
            update_tv_vod.vod_blurb = Set(vod_blurb);
            update = true;
        }
        if v.vod_remarks.is_empty() {
            update_tv_vod.vod_remarks = Set(vod_remarks);
            update = true;
        }
        if v.vod_content.is_empty() {
            update_tv_vod.vod_content = Set(vod_content);
            update = true;
        }
        if v.episode_count < (max_episodes) {
            update_tv_vod.episode_count = Set(max_episodes);
            update = true;
        }
        if update {
            update_tv_vod.update_time = Set(Local::now().naive_local());
            tv_vod_dao.update_by_id(update_tv_vod).await?;
        }
        if !pic.is_empty() {
            match tv_vod_pic_dao.find_by_collect_vod(collect_vod_id, v.id).await? {
                Some(p) => {
                    if p.pic != pic {
                        let tv_vod_pic = TvVodPicActiveModel {
                            id: Set(p.id),
                            pic: Set(pic),
                            ..Default::default()
                        };
                        tv_vod_pic_dao.update_by_id(tv_vod_pic).await?;
                    }
                },
                None => {
                    let tv_vod_pic = TvVodPicActiveModel {
                        id: Set(Uuid::now_v7()),
                        tv_vod_id: Set(v.id),
                        pic: Set(pic),
                        status: Set(true),
                        site_id: Set(site.id),
                        collect_vod_id: Set(collect_vod_id),
                    };
                    tv_vod_pic_dao.insert(tv_vod_pic).await?;
                }
            }
        }
    } else {
        let mut collect_type = HashSet::new();
        collect_type.insert(collect_type_id);
        let mut collect_vod = HashSet::new();
        collect_vod.insert(collect_vod_id);

        let collect_types = collect_type_dao.find_by_ids(collect_type.clone()).await?;
        let vod_show = collect_types.iter()
            .find(|&t| !t.show)
            .map(|_| false) // 如果有任意一个不显示，则视频不显示
            .unwrap_or(true);

        let tv_types = tv_type_bind_dao.find_by_collect_types(collect_vod.clone()).await?
            .iter()
            .map(|t|t.tv_type_id)
            .collect::<HashSet<Uuid>>();

        let tv_new_id = Uuid::now_v7();
        let insert_tv_vod = TvVodActiveModel {
            id: Set(tv_new_id),
            name: Set(vod_name),
            clear_name: Set(vod_clear_name),
            vod_tag: Set(vod_tag),
            vod_class: Set(vod_class),
            vod_actor: Set(vod_actor),
            vod_blurb: Set(vod_blurb),
            vod_remarks: Set(vod_remarks),
            vod_content: Set(vod_content),
            episode_count: Set(max_episodes),
            create_time: Set(create_time),
            update_time: Set(create_time),
            collect_type: Set(HashUUIDs(collect_type)),
            collect_vod: Set(HashUUIDs(collect_vod)),
            tv_type: Set(HashUUIDs(tv_types)),
            show: Set(vod_show),
        };
        tv_vod_dao.insert(insert_tv_vod).await?;

        if !pic.is_empty() {
            let tv_vod_pic = TvVodPicActiveModel {
                id: Set(Uuid::now_v7()),
                tv_vod_id: Set(tv_new_id),
                pic: Set(pic),
                status: Set(true),
                site_id: Set(site.id),
                collect_vod_id: Set(collect_vod_id),
            };
            tv_vod_pic_dao.insert(tv_vod_pic).await?;
        }
    };

    Ok(())
}

