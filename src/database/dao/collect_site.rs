use chrono::{Duration, Local};
use crate::common::model::Page;
use crate::common::ServerResult;
use crate::database::model::collect_site::{CollectSiteActiveModel, CollectSiteColumn, CollectSiteEntity, CollectSiteModel, CollectStatus};
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use uuid::Uuid;

pub struct CollectSiteDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> CollectSiteDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }
    
    pub async fn find_all(&self) -> ServerResult<Vec<CollectSiteModel>> {
        let list = CollectSiteEntity::find()
            .filter(CollectSiteColumn::Deleted.eq(false))
            .order_by_desc(CollectSiteColumn::CreateTime)
            .all(self.conn)
            .await?;
        Ok(list)
    }

    /// 根据ID查询
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<CollectSiteModel>> {
        let model = CollectSiteEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(model)
    }
    /// 根据ID查询
    pub async fn find_by_ids(&self, id: impl IntoIterator<Item = Uuid>) -> ServerResult<Vec<CollectSiteModel>> {
        let model = CollectSiteEntity::find()
            .filter(CollectSiteColumn::Id.is_in(id))
            .all(self.conn)
            .await?;
        Ok(model)
    }

    pub async fn find_undeleted_by_id(&self, id: Uuid) -> ServerResult<Option<CollectSiteModel>> {
        let model = CollectSiteEntity::find()
            .filter(CollectSiteColumn::Id.eq(id))
            .filter(CollectSiteColumn::Deleted.eq(false))
            .one(self.conn)
            .await?;
        Ok(model)
    }

    /// Insert
    pub async fn insert(&self, model: CollectSiteActiveModel) -> ServerResult<()> {
        CollectSiteEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID和版本修改
    pub async fn update_by_version(&self, record: CollectSiteActiveModel, id: Uuid, version: i32) -> ServerResult<u64> {
        let res = CollectSiteEntity::update_many()
            .set(record)
            .filter(CollectSiteColumn::Id.eq(id))
            .filter(CollectSiteColumn::Version.eq(version))
            .exec(self.conn)
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn update_collect_finish(&self, full: bool, success: bool, id: Uuid) -> ServerResult<()> {
        let model = if full {
            CollectSiteActiveModel {
                id: Set(id),
                full_status: if success { Set(CollectStatus::Completed) } else { Set(CollectStatus::Failed) },
                full_collect_time: Set(Local::now().naive_local()),
                status: if success { Set(true) } else { NotSet },
                last_time: Set(Local::now().naive_local()),
                ..Default::default()
            }
        } else {
            CollectSiteActiveModel {
                id: Set(id),
                collect_status: if success { Set(CollectStatus::Completed) } else { Set(CollectStatus::Failed) },
                last_time: Set(Local::now().naive_local()),
                ..Default::default()
            }
        };
        CollectSiteEntity::update(model).exec(self.conn).await?;
        Ok(())
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<CollectSiteModel>, u64)> {
        let total = CollectSiteEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = CollectSiteEntity::find()
            .filter(condition)
            .order_by_desc(CollectSiteColumn::CreateTime)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }

    pub async fn find_task_site(&self) -> ServerResult<Vec<CollectSiteModel>> {
        let interval = Duration::hours(6);
        let list = CollectSiteEntity::find()
            .filter(CollectSiteColumn::FullStatus.eq(CollectStatus::Completed))
            .filter(CollectSiteColumn::CollectStatus.ne(CollectStatus::Processing))
            .filter(CollectSiteColumn::Status.eq(true))
            .filter(CollectSiteColumn::LastTime.lte(Local::now().naive_local() - interval))
            .all(self.conn)
            .await?;
        Ok(list)
    }
}
