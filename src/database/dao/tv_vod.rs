use crate::common::model::Page;
use crate::common::ServerResult;
use crate::database::model::tv_vod::{TvVodActiveModel, TvVodColumn, TvVodEntity, TvVodModel};
use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::{PaginatorTrait, Set};
use uuid::Uuid;

pub struct TvVodDao<'d, C: ConnectionTrait> {
    conn: &'d C
}

impl<'d, C: ConnectionTrait> TvVodDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }


    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<TvVodModel>, u64)> {
        let total = TvVodEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }
        let list = TvVodEntity::find()
            .filter(condition)
            .order_by_desc(TvVodColumn::CreateTime)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;
        Ok((list, total))
    }

    pub async fn find_page_by_collect_type_id(&self, page: Page, collect_type_id: Uuid) -> ServerResult<Vec<TvVodModel>> {
        let mut conditions = Condition::all();
        let array_literal = format!(r#" "collect_type" ?| ARRAY['{collect_type_id}'] "#);
        conditions = conditions.add(Expr::cust(array_literal));
        let list = TvVodEntity::find()
            .filter(conditions)
            .order_by_asc(TvVodColumn::Id)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok(list)
    }

    /// 根据名称查询
    pub async fn find_by_clear_name(&self, name: &str) -> ServerResult<Option<TvVodModel>> {
        let vod = TvVodEntity::find()
            .filter(TvVodColumn::ClearName.eq(name))
            .one(self.conn)
            .await?;
        Ok(vod)
    }

    /// 根据ID查询
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<TvVodModel>> {
        let vod = TvVodEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(vod)
    }

    pub async fn find_by_ids(&self, ids: impl IntoIterator<Item = Uuid>) -> ServerResult<Vec<TvVodModel>> {
        let vod = TvVodEntity::find()
            .filter(TvVodColumn::Id.is_in(ids))
            .all(self.conn)
            .await?;
        Ok(vod)
    }

    /// 新增
    pub async fn insert(&self, record: TvVodActiveModel) -> ServerResult<()> {
        TvVodEntity::insert(record)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID修改
    pub async fn update_by_id(&self, record: TvVodActiveModel) -> ServerResult<()> {
        TvVodEntity::update(record).exec(self.conn).await?;
        Ok(())
    }

    /// 根据ID修改显示状态
    pub async fn update_show(&self, id: Uuid, show: bool) -> ServerResult<()> {
        let update = TvVodActiveModel {
            id: Set(id),
            show: Set(show),
            ..Default::default()
        };
        TvVodEntity::update(update).exec(self.conn).await?;
        Ok(())
    }
}
