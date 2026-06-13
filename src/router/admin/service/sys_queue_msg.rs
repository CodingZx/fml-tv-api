use crate::common::model::{Page, Pager};
use crate::common::pgq::dao::SysQueueMsgDao;
use crate::common::pgq::entity::message::SysQueueMsgColumn;
use crate::common::pgq::MessageStatus;
use crate::common::state::AppState;
use crate::common::ServerResult;
use crate::router::admin::vo::sys_queue_msg::{SysQueueMsgListReq, SysQueueMsgListResp};
use sea_orm::Condition;
use sea_orm::{ColumnTrait, TransactionTrait};
use std::sync::Arc;
use uuid::Uuid;

pub struct SysQueueMsgService {
    state: Arc<AppState>,
}

impl SysQueueMsgService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// 查询列表
    pub async fn list(&self, param: SysQueueMsgListReq) -> ServerResult<Pager<SysQueueMsgListResp>> {
        let mut conditions = Condition::all();
        let status = MessageStatus::from_value(param.status);
        if let Some(s) = status {
            conditions = conditions.add(SysQueueMsgColumn::Status.eq(s));
        }
        let queue = param.queue.unwrap_or_default();
        if !queue.is_empty() {
            conditions = conditions.add(SysQueueMsgColumn::Queue.contains(&queue));
        }
        let page = Page::from(param.page, param.size);

        let (records, count) = SysQueueMsgDao::new(&self.state.db)
            .find_page_list(page, conditions)
            .await?;

        let result = records
            .into_iter()
            .map(SysQueueMsgListResp::new)
            .collect::<Vec<_>>();

        Ok(Pager::new(result, count))
    }

    /// 重发
    pub async fn resend(&self, ids: Vec<Uuid>) -> ServerResult<()> {
        self.state.db.transaction(|tx|{
            Box::pin(async move {
                let dao = SysQueueMsgDao::new(tx);
                for id in ids {
                    dao.update_pending(id).await?;
                }
                Ok(())
            })
        }).await?;
        Ok(())
    }

    /// 删除
    pub async fn delete(&self, ids: Vec<Uuid>) -> ServerResult<()> {
        SysQueueMsgDao::new(&self.state.db).delete_by_ids(ids).await?;
        Ok(())
    }
}