use crate::common::model::Page;
use crate::common::ServerResult;
use crate::database::model::tv_group::{TvGroupActiveModel, TvGroupColumn, TvGroupEntity, TvGroupModel};
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

pub struct TvGroupDao<'d, C: ConnectionTrait> {
    conn: &'d C
}

impl<'d, C: ConnectionTrait> TvGroupDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<TvGroupModel>, u64)> {
        let total = TvGroupEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = TvGroupEntity::find()
            .filter(condition)
            .order_by_asc(TvGroupColumn::SortNum)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }

    pub async fn find_all(&self) -> ServerResult<Vec<TvGroupModel>> {
        let typ = TvGroupEntity::find()
            .order_by_asc(TvGroupColumn::SortNum)
            .all(self.conn)
            .await?;
        Ok(typ)
    }
    
    /// 根据名称查询
    pub async fn find_by_name(&self, name: &str) -> ServerResult<Option<TvGroupModel>> {
        let typ = TvGroupEntity::find()
            .filter(TvGroupColumn::Name.eq(name))
            .one(self.conn)
            .await?;
        Ok(typ)
    }

    /// 根据ID查询类型
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<TvGroupModel>> {
        let typ = TvGroupEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(typ)
    }

    /// 新增
    pub async fn insert(&self, record: TvGroupActiveModel) -> ServerResult<()> {
        TvGroupEntity::insert(record)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID和版本修改
    pub async fn update_by_version(&self, record: TvGroupActiveModel, id: Uuid, version: i32) -> ServerResult<u64> {
        let res = TvGroupEntity::update_many()
            .set(record)
            .filter(TvGroupColumn::Id.eq(id))
            .filter(TvGroupColumn::Version.eq(version))
            .exec(self.conn)
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn delete_by_ids(&self, id: impl IntoIterator<Item = Uuid>) -> ServerResult<()> {
        TvGroupEntity::delete_many()
            .filter(TvGroupColumn::Id.is_in(id))
            .exec(self.conn).await?;
        Ok(())
    }
}

