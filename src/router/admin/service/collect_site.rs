use crate::common::error::ServerError::{BusinessError, BusinessStrError, OptimisticLock};
use crate::common::model::{Page, Pager};
use crate::common::pgq::service::MessageSender;
use crate::common::state::AppState;
use crate::common::token::AdminToken;
use crate::common::util::collect_util;
use crate::common::{consts, ServerResult};
use crate::consumer::collect_consumer::CollectMessage;
use crate::database::dao::collect_site::CollectSiteDao;
use crate::database::model::collect_site::{CollectSiteActiveModel, CollectSiteColumn, CollectStatus};
use crate::router::admin::vo::collect_site::{CollectSiteCollectReq, CollectSiteDeleteReq, CollectSiteFullCollectReq, CollectSiteListReq, CollectSiteListResp, CollectSiteSaveReq, CollectSiteStatusReq};
use crate::router::admin::vo::ComBoxResp;
use chrono::Local;
use sea_orm::{ColumnTrait, Condition, NotSet, Set, TransactionTrait};
use std::sync::Arc;
use uuid::Uuid;

pub struct CollectSiteService {
    state: Arc<AppState>,
}

impl CollectSiteService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn all(&self) -> ServerResult<Vec<ComBoxResp>> {
        let records = CollectSiteDao::new(&self.state.db)
            .find_all()
            .await?
            .into_iter()
            .map(|r| ComBoxResp::from(r.id, r.name))
            .collect();
        Ok(records)
    }

    /// 获得列表
    pub async fn list(&self, param: CollectSiteListReq) -> ServerResult<Pager<CollectSiteListResp>> {
        let mut conditions = Condition::all();
        let name = param.name.unwrap_or_default();
        if !name.is_empty() {
            conditions = conditions.add(CollectSiteColumn::Name.contains(&name));
        }
        let main_page = param.main_page.unwrap_or_default();
        if !main_page.is_empty() {
            conditions = conditions.add(CollectSiteColumn::MainPage.contains(&main_page));
        }

        conditions = conditions.add(CollectSiteColumn::Deleted.eq(false));

        let page = Page::from(param.page, param.size);
        let (records, total) = CollectSiteDao::new(&self.state.db).find_page_list(page, conditions).await?;

        let result = records
            .into_iter()
            .map(CollectSiteListResp::new)
            .collect();

        Ok(Pager::new(result, total))
    }

    /// 保存类型
    pub async fn save(&self, token: AdminToken, param: CollectSiteSaveReq) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let dao = CollectSiteDao::new(&self.state.db);

        // 尝试获取list接口
        if let Err(e) = collect_util::fetch_list(&param.req_url).await {
            let err = format!("尝试拉取list接口失败, 请确认请求地址是否正确, 错误内容:{e}");
            return Err(BusinessStrError(err));
        }

        let mut model = CollectSiteActiveModel {
            id: NotSet,
            name: Set(param.name),
            main_page: Set(param.main_page),
            req_url: Set(param.req_url),
            full_status: NotSet,
            full_collect_time: NotSet,
            collect_status: NotSet,
            player: Set(param.player),
            last_time: NotSet,
            status: NotSet,
            update_time: Set(now),
            update_user: Set(token.account_id()),
            ..Default::default()
        };
        match param.id {
            Some(id) => {
                if dao.find_undeleted_by_id(id).await?.is_none() {
                    return Err(BusinessError("ID错误, 数据不存在"))
                }
                // 修改
                model.version = Set(param.version + 1);
                let row = dao.update_by_version(model, id, param.version).await?;
                if row == 0 {
                    return Err(OptimisticLock)
                }
            },
            None => {
                // 新增
                model.id = Set(Uuid::now_v7());
                model.create_user = Set(token.account_id());
                model.create_time = Set(now);
                model.version = Set(1);
                model.full_status = Set(CollectStatus::Waiting);
                model.full_collect_time = Set(consts::get_default_time());
                model.collect_status = Set(CollectStatus::Waiting);
                model.last_time = Set(consts::get_default_time());
                model.status = Set(false);
                model.login_delete_created();
                dao.insert(model).await?;
            }
        }
        Ok(())
    }

    pub async fn send_full_collect(&self, token: AdminToken, param: CollectSiteFullCollectReq) -> ServerResult<()> {
        let collect_site = CollectSiteDao::new(&self.state.db).find_undeleted_by_id(param.id).await?.ok_or(BusinessError("ID错误, 数据不存在"))?;
        if collect_site.full_status == CollectStatus::Processing {
            return Err(BusinessError("全量采集任务正在进行中, 请勿重复提交"));
        }
        if collect_site.collect_status == CollectStatus::Processing {
            return Err(BusinessError("增量采集任务正在进行中, 请勿重复提交"));
        }
        self.state.db.transaction(|tx| {
            Box::pin(async move {
                let update = CollectSiteActiveModel {
                    full_status: Set(CollectStatus::Processing),
                    status: Set(false),
                    update_time: Set(Local::now().naive_local()),
                    update_user: Set(token.account_id()),
                    ..Default::default()
                };
                let rows = CollectSiteDao::new(tx).update_by_version(update, param.id, param.version).await?;
                if rows == 0 {
                    return Err(OptimisticLock);
                }

                let msg = CollectMessage::full(param.id);
                MessageSender::new(tx).send_json(consts::queues::COLLECT_QUEUE, msg).await?;
                Ok(())
            })
        }).await?;
        Ok(())
    }

    pub async fn send_collect(&self, token: AdminToken, param: CollectSiteCollectReq) -> ServerResult<()> {
        let collect_site = CollectSiteDao::new(&self.state.db).find_undeleted_by_id(param.id).await?.ok_or(BusinessError("ID错误, 数据不存在"))?;
        if !collect_site.status {
            return Err(BusinessError("当前采集站已被禁用, 无法发起增量采集"));
        }
        if collect_site.full_status == CollectStatus::Processing {
            return Err(BusinessError("全量采集任务正在进行中, 请勿重复提交"));
        }
        if collect_site.collect_status == CollectStatus::Processing {
            return Err(BusinessError("增量采集任务正在进行中, 请勿重复提交"));
        }
        self.state.db.transaction(|tx| {
            Box::pin(async move {
                let update = CollectSiteActiveModel {
                    collect_status: Set(CollectStatus::Processing),
                    update_time: Set(Local::now().naive_local()),
                    update_user: Set(token.account_id()),
                    ..Default::default()
                };
                let rows = CollectSiteDao::new(tx).update_by_version(update, param.id, param.version).await?;
                if rows == 0 {
                    return Err(OptimisticLock)
                }

                let msg = CollectMessage::hour(param.id, param.hour);
                MessageSender::new(tx).send_json(consts::queues::COLLECT_QUEUE, msg).await?;
                Ok(())
            })
        }).await?;
        Ok(())
    }

    /// 修改状态
    pub async fn update_status(&self, token: AdminToken, param: CollectSiteStatusReq) -> ServerResult<()> {
        let dao = CollectSiteDao::new(&self.state.db);
        let model = dao.find_by_id(param.id).await?.filter(|r| !r.deleted).ok_or(BusinessError("ID错误, 数据不存在"))?;
        if model.full_status != CollectStatus::Completed {
            return Err(BusinessError("请等待全量采集完成"));
        }
        let model = CollectSiteActiveModel {
            status: Set(param.status),
            update_user: Set(token.account_id()),
            update_time: Set(Local::now().naive_local()),
            version: Set(param.version + 1),
            ..Default::default()
        };
        let rows = dao.update_by_version(model, param.id, param.version).await?;
        if rows == 0 {
            return Err(OptimisticLock);
        }
        Ok(())
    }

    /// 删除
    pub async fn delete(&self, token: AdminToken, param: CollectSiteDeleteReq) -> ServerResult<()> {
        self.state.db.transaction(|tx| {
            Box::pin(async move {
                let type_dao = CollectSiteDao::new(tx);
                for it in param.id_ver {
                    let mut model = CollectSiteActiveModel {
                        version: Set(it.version + 1),
                        ..Default::default()
                    };
                    model.login_deleted(token.account_id(), param.reason.clone());
                    let rows = type_dao.update_by_version(model, it.id, it.version).await?;
                    if rows == 0 {
                        return Err(OptimisticLock);
                    }
                }
                Ok(())
            })
        }).await?;
        Ok(())
    }

}
