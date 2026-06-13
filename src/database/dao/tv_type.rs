use crate::common::model::Page;
use crate::common::ServerResult;
use crate::database::model::tv_type::{TvTypeActiveModel, TvTypeColumn, TvTypeEntity, TvTypeModel};
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

pub struct TvTypeDao<'d, C: ConnectionTrait> {
    conn: &'d C
}

impl<'d, C: ConnectionTrait> TvTypeDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<TvTypeModel>, u64)> {
        let total = TvTypeEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = TvTypeEntity::find()
            .filter(condition)
            .order_by_asc(TvTypeColumn::SortNum)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }

    /// 根据名称查询
    pub async fn find_by_type_name(&self, name: &str) -> ServerResult<Option<TvTypeModel>> {
        let typ = TvTypeEntity::find()
            .filter(TvTypeColumn::Name.eq(name))
            .one(self.conn)
            .await?;
        Ok(typ)
    }

    pub async fn find_all(&self) -> ServerResult<Vec<TvTypeModel>> {
        let all = TvTypeEntity::find()
            .order_by_asc(TvTypeColumn::SortNum)
            .all(self.conn)
            .await?;
        Ok(all)
    }

    /// 根据ID查询类型
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<TvTypeModel>> {
        let typ = TvTypeEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(typ)
    }

    /// 根据ID查询类型
    pub async fn find_by_ids(&self, ids: impl IntoIterator<Item = Uuid>) -> ServerResult<Vec<TvTypeModel>> {
        let typ = TvTypeEntity::find()
            .filter(TvTypeColumn::Id.is_in(ids))
            .order_by_asc(TvTypeColumn::SortNum)
            .all(self.conn)
            .await?;
        Ok(typ)
    }

    /// 新增
    pub async fn insert(&self, record: TvTypeActiveModel) -> ServerResult<()> {
        TvTypeEntity::insert(record)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID和版本修改
    pub async fn update_by_version(&self, record: TvTypeActiveModel, id: Uuid, version: i32) -> ServerResult<u64> {
        let res = TvTypeEntity::update_many()
            .set(record)
            .filter(TvTypeColumn::Id.eq(id))
            .filter(TvTypeColumn::Version.eq(version))
            .exec(self.conn)
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> ServerResult<()> {
        TvTypeEntity::delete_by_id(id).exec(self.conn).await?;
        Ok(())
    }
}

