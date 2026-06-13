use crate::common::model::{Page, Pager};
use crate::common::state::AppState;
use crate::common::ServerResult;
use crate::router::admin::vo::sys_login_log::{SysLoginLogListReq, SysLoginLogListResp};
use crate::database::dao::sys_login_log::SysLoginLogDao;
use crate::database::model::sys_login_log::SysLoginLogColumn;
use sea_orm::{ColumnTrait, Condition};
use std::sync::Arc;
use uuid::Uuid;

pub struct SysLoginLogService {
    state: Arc<AppState>,
}

impl SysLoginLogService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// 查询列表
    pub async fn list(&self, param: SysLoginLogListReq) -> ServerResult<Pager<SysLoginLogListResp>> {
        let mut conditions = Condition::all();
        let ip = param.ip.unwrap_or_default();
        if !ip.is_empty() {
            conditions = conditions.add(SysLoginLogColumn::IpAddr.contains(&ip));
        }
        let user_name = param.user_name.unwrap_or_default();
        if !user_name.is_empty() {
            conditions = conditions.add(SysLoginLogColumn::UserName.contains(&user_name));
        }

        let page = Page::from(param.page, param.size);
        let (records, count) = SysLoginLogDao::new(&self.state.db)
            .find_page_list(page, conditions)
            .await?;

        let result = records
            .into_iter()
            .map(SysLoginLogListResp::new)
            .collect();
        Ok(Pager::new(result, count))
    }

    /// 删除
    pub async fn delete(&self, id: Vec<Uuid>) -> ServerResult<()> {
        SysLoginLogDao::new(&self.state.db).delete_by_ids(id).await?;
        Ok(())
    }
    
    /// 清空
    pub async fn clear(&self) -> ServerResult<()> {
        SysLoginLogDao::new(&self.state.db).clear().await?;
        Ok(())
    }
}