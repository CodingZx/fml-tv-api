use crate::common::model::Page;
use crate::common::pgq::entity::delay::{SysQueueDelayMsgActiveModel, SysQueueDelayMsgColumn, SysQueueDelayMsgEntity};
use crate::common::pgq::entity::message::{SysQueueMsgActiveModel, SysQueueMsgColumn, SysQueueMsgEntity, SysQueueMsgModel};
use crate::common::pgq::{DelayMessageBody, MessageBody, MessageStatus};
use crate::common::ServerResult;
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Set, Statement, Value};
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use uuid::Uuid;

pub struct SysQueueMsgDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysQueueMsgDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 获取一条消息
    pub async fn take_one_msg(&self, queue: &str) -> ServerResult<Option<MessageBody>> {
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT id, message FROM sys_queue_msg
            WHERE queue = $1 and status = 0
            ORDER BY publish_time ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
            vec![Value::String(Some(queue.to_string()))]
        );

        let body = MessageBody::find_by_statement(stmt).one(self.conn).await?;
        Ok(body)
    }
    
    /// 根据ID查询
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<SysQueueMsgModel>> {
        let model = SysQueueMsgEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(model)
    }


    /// 保存新的消息
    pub async fn insert(&self, model: SysQueueMsgActiveModel) -> ServerResult<()> {
        SysQueueMsgEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 批量保存消息
    pub async fn batch_insert(&self, model: Vec<SysQueueMsgActiveModel>) -> ServerResult<()> {
        SysQueueMsgEntity::insert_many(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 更新为等待执行
    pub async fn update_pending(&self, id: Uuid) -> ServerResult<()> {
        SysQueueMsgEntity::update(SysQueueMsgActiveModel {
            id: Set(id),
            status: Set(MessageStatus::Pending),
            publish_time: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        }).exec(self.conn).await?;
        Ok(())
    }

    /// 更新为执行中
    pub async fn update_processing(&self, id: Uuid) -> ServerResult<()> {
        SysQueueMsgEntity::update(SysQueueMsgActiveModel {
            id: Set(id),
            status: Set(MessageStatus::Processing),
            process_time: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        }).exec(self.conn).await?;
        Ok(())
    }

    /// 更新为执行成功
    pub async fn update_success(&self, id: Uuid) -> ServerResult<()> {
        SysQueueMsgEntity::update(SysQueueMsgActiveModel {
            id: Set(id),
            status: Set(MessageStatus::Success),
            finish_time: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        }).exec(self.conn).await?;
        Ok(())
    }

    /// 更新为执行失败
    pub async fn update_failed(&self, id: Uuid, error: String) -> ServerResult<()> {
        SysQueueMsgEntity::update(SysQueueMsgActiveModel {
            id: Set(id),
            status: Set(MessageStatus::Failed),
            finish_time: Set(chrono::Local::now().naive_local()),
            error_detail: Set(error),
            ..Default::default()
        }).exec(self.conn).await?;
        Ok(())
    }
    
    /// 根据ID删除
    pub async fn delete_by_id(&self, id: Uuid) -> ServerResult<()> {
        SysQueueMsgEntity::delete_by_id(id).exec(self.conn).await?;
        Ok(())
    }

    /// 根据ID删除
    pub async fn delete_by_ids(&self, id: Vec<Uuid>) -> ServerResult<()> {
        SysQueueMsgEntity::delete_many().filter(SysQueueMsgColumn::Id.is_in(id)).exec(self.conn).await?;
        Ok(())
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<SysQueueMsgModel>, u64)> {
        let total = SysQueueMsgEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = SysQueueMsgEntity::find()
            .filter(condition)
            .order_by_desc(SysQueueMsgColumn::PublishTime)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }
}


pub struct SysQueueDelayMsgDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysQueueDelayMsgDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 获取延迟消息
    pub async fn take_delay_msg(&self) -> ServerResult<Vec<DelayMessageBody>> {
        let now = chrono::Local::now().naive_local();
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT id, queue, message FROM sys_queue_delay_msg
            WHERE delay_time <= $1
            ORDER BY delay_time ASC
            LIMIT 10
            FOR UPDATE SKIP LOCKED
            "#,
            vec![Value::ChronoDateTime(Some(now))]
        );

        let delay_msg = DelayMessageBody::find_by_statement(stmt).all(self.conn).await?;
        Ok(delay_msg)
    }

    /// 保存新的消息
    pub async fn insert(&self, model: SysQueueDelayMsgActiveModel) -> ServerResult<()> {
        SysQueueDelayMsgEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 删除消息
    pub async fn delete_by_ids(&self, id: Vec<Uuid>) -> ServerResult<()> {
        SysQueueDelayMsgEntity::delete_many()
            .filter(SysQueueDelayMsgColumn::Id.is_in(id))
            .exec(self.conn).await?;
        Ok(())
    }
}
