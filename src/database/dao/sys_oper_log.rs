use crate::common::model::Page;
use crate::common::ServerResult;
use crate::database::model::sys_oper_log::{SysOperLogActiveModel, SysOperLogColumn, SysOperLogEntity, SysOperLogModel};
use sea_orm::{ColumnTrait, ConnectionTrait, QueryFilter};
use sea_orm::{Condition, EntityTrait};
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use uuid::Uuid;

pub struct SysOperLogDao<'d, C: ConnectionTrait> {
    conn: &'d C
}

impl<'d, C: ConnectionTrait> SysOperLogDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<SysOperLogModel>, u64)> {
        let total = SysOperLogEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = SysOperLogEntity::find()
            .filter(condition)
            .order_by_desc(SysOperLogColumn::CreateTime)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }
    /// 根据ID查询
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<SysOperLogModel>> {
        let log = SysOperLogEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(log)
    }

    /// 新增
    pub async fn insert(&self, model: SysOperLogActiveModel) -> ServerResult<()> {
        SysOperLogEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 删除
    pub async fn delete_by_ids(&self, id: Vec<Uuid>) -> ServerResult<()> {
        SysOperLogEntity::delete_many()
            .filter(SysOperLogColumn::Id.is_in(id))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 清空
    pub async fn clear(&self) -> ServerResult<()> {
        SysOperLogEntity::delete_many()
            .exec(self.conn)
            .await?;
        Ok(())
    }
}
