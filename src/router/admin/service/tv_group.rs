use crate::common::error::ServerError::{BusinessError, OptimisticLock};
use crate::common::model::{IdsReq, Page, Pager};
use crate::common::state::AppState;
use crate::common::token::AdminToken;
use crate::common::ServerResult;
use crate::database::dao::tv_group::TvGroupDao;
use crate::database::dao::tv_type::TvTypeDao;
use crate::database::model::tv_group::{TvGroupActiveModel, TvGroupColumn};
use crate::database::model::VecUUIDs;
use crate::router::admin::vo::tv_group::{TvGroupListReq, TvGroupListResp, TvGroupSaveReq};
use chrono::Local;
use sea_orm::{ColumnTrait, Condition, NotSet, Set, TransactionTrait};
use std::sync::Arc;
use uuid::Uuid;

pub struct TvGroupService {
    state: Arc<AppState>,
}

impl TvGroupService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }


    /// 获得列表
    pub async fn list(&self, param: TvGroupListReq) -> ServerResult<Pager<TvGroupListResp>> {
        let mut conditions = Condition::all();
        let name = param.name.unwrap_or_default();
        if !name.is_empty() {
            conditions = conditions.add(TvGroupColumn::Name.contains(name));
        }

        let page = Page::from(param.page, param.size);
        let (records, total) = TvGroupDao::new(&self.state.db).find_page_list(page, conditions).await?;

        let mut result = Vec::new();
        for record in records {
            let type_ids = record.types.0.clone();
            let types = if type_ids.len() > 0 {
                TvTypeDao::new(&self.state.db).find_by_ids(type_ids).await?
            } else {
                vec![]
            };
            result.push(TvGroupListResp::new(record, types))
        }

        Ok(Pager::new(result, total))
    }

    /// 保存类型
    pub async fn save(&self, token: AdminToken, param: TvGroupSaveReq) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let dao = TvGroupDao::new(&self.state.db);
        let check_name = dao.find_by_name(&param.name).await?;

        let mut model = TvGroupActiveModel {
            id: NotSet,
            name: Set(param.name),
            types: Set(VecUUIDs(param.type_ids)),
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
                        let row = TvGroupDao::new(tx).update_by_version(model, id, param.version).await?;
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
                        TvGroupDao::new(tx).insert(model).await?;
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
                TvGroupDao::new(tx).delete_by_ids(param.id).await?;
                Ok(())
            })
        }).await?;
        Ok(())
    }
}
