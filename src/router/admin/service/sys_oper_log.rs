use crate::common::error::ServerError::BusinessError;
use crate::common::model::{Page, Pager};
use crate::common::state::AppState;
use crate::common::util::time_util::TimeUtil;
use crate::common::ServerResult;
use crate::router::admin::vo::sys_oper_log::{SysOperLogDetailResp, SysOperLogListReq, SysOperLogListResp};
use crate::database::dao::sys_account::SysAccountDao;
use crate::database::dao::sys_oper_log::SysOperLogDao;
use crate::database::model::sys_oper_log::SysOperLogColumn;
use sea_orm::{ColumnTrait, Condition};
use std::sync::Arc;
use uuid::Uuid;

pub struct SysOperLogService {
    state: Arc<AppState>
}

impl SysOperLogService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// 查询列表
    pub async fn list(&self, param: SysOperLogListReq) -> ServerResult<Pager<SysOperLogListResp>> {
        let mut conditions = Condition::all();
        if let Some(start) = param.start {
            if let Some(v) = TimeUtil::parse_default(&start) {
                conditions = conditions.add(SysOperLogColumn::CreateTime.gt(v));
            }
        }
        if let Some(end) = param.end {
            if let Some(v) = TimeUtil::parse_default(&end) {
                conditions = conditions.add(SysOperLogColumn::CreateTime.lt(v));
            }
        }
        let page = Page::from(param.page, param.size);
        let (records, count) = SysOperLogDao::new(&self.state.db).find_page_list(page, conditions).await?;

        let account_dao = SysAccountDao::new(&self.state.db);
        let mut result = Vec::with_capacity(records.len());
        for log in records {
            let account_name = if let Some(id) = log.oper_user {
                account_dao.find_by_id(id).await?.map(|it|it.real_name).unwrap_or_default()
            } else {
                String::new()
            };
            result.push(SysOperLogListResp::new(log, account_name));
        }

        Ok(Pager::new(result, count))
    }

    /// 查询详情
    pub async fn detail(&self, id: Uuid) -> ServerResult<SysOperLogDetailResp> {
        let log = SysOperLogDao::new(&self.state.db).find_by_id(id).await?.ok_or(BusinessError("数据ID错误"))?;
        let account_name = if let Some(id) = log.oper_user {
            SysAccountDao::new(&self.state.db).find_by_id(id).await?.map(|it|it.real_name).unwrap_or_default()
        } else {
            String::new()
        };
        Ok(SysOperLogDetailResp::new(log, account_name))
    }

    /// 删除
    pub async fn delete(&self, id: Vec<Uuid>) -> ServerResult<()> {
        SysOperLogDao::new(&self.state.db).delete_by_ids(id).await?;
        Ok(())
    }

    /// 清空
    pub async fn clear(&self) -> ServerResult<()> {
        SysOperLogDao::new(&self.state.db).clear().await?;
        Ok(())
    }
}
