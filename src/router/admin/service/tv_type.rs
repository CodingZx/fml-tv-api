use crate::common::error::ServerError::{BusinessError, OptimisticLock};
use crate::common::model::{IdsReq, Page, Pager};
use crate::common::state::AppState;
use crate::common::token::AdminToken;
use crate::common::ServerResult;
use crate::database::dao::tv_type::TvTypeDao;
use crate::database::dao::tv_type_bind::TvTypeBindDao;
use crate::database::model::tv_type::{TvTypeActiveModel, TvTypeColumn};
use crate::router::admin::vo::tv_type::{TvTypeListReq, TvTypeListResp, TvTypeSaveReq};
use crate::router::admin::vo::ComBoxResp;
use chrono::Local;
use sea_orm::{ColumnTrait, Condition, NotSet, Set, TransactionTrait};
use std::sync::Arc;
use uuid::Uuid;

pub struct TvTypeService {
    state: Arc<AppState>,
}

impl TvTypeService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn all(&self) -> ServerResult<Vec<ComBoxResp>> {
        let result = TvTypeDao::new(&self.state.db)
            .find_all()
            .await?
            .into_iter()
            .map(|t| ComBoxResp::from(t.id, t.name))
            .collect::<Vec<_>>();
        Ok(result)
    }

    /// 获得列表
    pub async fn list(&self, param: TvTypeListReq) -> ServerResult<Pager<TvTypeListResp>> {
        let mut conditions = Condition::all();
        let name = param.name.unwrap_or_default();
        if !name.is_empty() {
            conditions = conditions.add(TvTypeColumn::Name.contains(name));
        }

        let page = Page::from(param.page, param.size);
        let (records, total) = TvTypeDao::new(&self.state.db).find_page_list(page, conditions).await?;

        let result = records
            .into_iter()
            .map(TvTypeListResp::new)
            .collect();

        Ok(Pager::new(result, total))
    }

    /// 保存类型
    pub async fn save(&self, token: AdminToken, param: TvTypeSaveReq) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let dao = TvTypeDao::new(&self.state.db);
        let check_name = dao.find_by_type_name(&param.name).await?;

        let mut model = TvTypeActiveModel {
            id: NotSet,
            name: Set(param.name),
            sort_num: Set(param.sort_num),
            update_time: Set(now),
            update_user: Set(token.account_id()),
            ..Default::default()
        };
        match param.id {
            Some(id) => {
                if dao.find_by_id(id).await?.is_none() {
                    return Err(BusinessError("ID错误, 数据不存在"))
                }
                if check_name.map(|r| r.id != id).unwrap_or(false) {
                    return Err(BusinessError("类型名称重复"));
                }

                // 修改
                model.version = Set(param.version + 1);
                self.state.db.transaction(|tx| {
                    Box::pin(async move {
                        let row = TvTypeDao::new(tx).update_by_version(model, id, param.version).await?;
                        if row == 0 {
                            return Err(OptimisticLock)
                        }
                        Ok(())
                    })
                }).await?;
            },
            None => {
                if check_name.is_some() {
                    return Err(BusinessError("类型名称重复"));
                }
                // 新增
                model.id = Set(Uuid::now_v7());
                model.create_user = Set(token.account_id());
                model.create_time = Set(now);
                model.version = Set(1);

                self.state.db.transaction(|tx| {
                    Box::pin(async move {
                        TvTypeDao::new(tx).insert(model).await?;
                        Ok(())
                    })
                }).await?;
            }
        }
        Ok(())
    }

    /// 删除
    pub async fn delete(&self, param: IdsReq) -> ServerResult<()> {
        self.state.db.transaction(|tx| {
            Box::pin(async move {
                let type_dao = TvTypeDao::new(tx);
                let type_bind_dao = TvTypeBindDao::new(tx);
                for it in param.id {
                    type_dao.delete_by_id(it).await?;
                    type_bind_dao.delete_by_collect_type_id(it).await?;
                }
                Ok(())
            })
        }).await?;
        Ok(())
    }
}
