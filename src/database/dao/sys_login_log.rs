use crate::common::model::Page;
use crate::common::ServerResult;
use crate::database::model::sys_login_log::{SysLoginLogActiveModel, SysLoginLogColumn, SysLoginLogEntity, SysLoginLogModel};
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

pub struct SysLoginLogDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysLoginLogDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// Insert
    pub async fn insert(&self, model: SysLoginLogActiveModel) -> ServerResult<()> {
        SysLoginLogEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<SysLoginLogModel>, u64)> {
        let total = SysLoginLogEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = SysLoginLogEntity::find()
            .filter(condition)
            .order_by_desc(SysLoginLogColumn::CreateTime)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }

    /// 删除
    pub async fn delete_by_ids(&self, id: Vec<Uuid>) -> ServerResult<()> {
        SysLoginLogEntity::delete_many()
            .filter(SysLoginLogColumn::Id.is_in(id))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 清空
    pub async fn clear(&self) -> ServerResult<()> {
        SysLoginLogEntity::delete_many().exec(self.conn).await?;
        Ok(())
    }

}
