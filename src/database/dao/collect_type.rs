use crate::common::ServerResult;
use crate::database::model::collect_type::{CollectTypeActiveModel, CollectTypeColumn, CollectTypeEntity, CollectTypeModel};
use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter, QueryOrder, Set};
use uuid::Uuid;

pub struct CollectTypeDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> CollectTypeDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 根据ID查询
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<CollectTypeModel>> {
        let model = CollectTypeEntity::find_by_id(id)
            .one(self.conn)
            .await?;

        Ok(model)
    }

    /// Insert
    pub async fn insert(&self, model: CollectTypeActiveModel) -> ServerResult<()> {
        CollectTypeEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 查询类别
    pub async fn find_by_type_id(&self, site_id: Uuid, type_id: i32) -> ServerResult<Option<CollectTypeModel>> {
        let model = CollectTypeEntity::find()
            .filter(CollectTypeColumn::SiteId.eq(site_id))
            .filter(CollectTypeColumn::TypeId.eq(type_id))
            .one(self.conn)
            .await?;
        Ok(model)
    }

    /// 查询列表
    pub async fn find_by_site_id(&self, site_id: Uuid) -> ServerResult<Vec<CollectTypeModel>> {
        let list = CollectTypeEntity::find()
            .filter(CollectTypeColumn::SiteId.eq(site_id))
            .order_by_asc(CollectTypeColumn::TypeId)
            .all(self.conn)
            .await?;

        Ok(list)
    }

    pub async fn find_by_ids(&self, collect_type_ids: impl IntoIterator<Item = Uuid>) -> ServerResult<Vec<CollectTypeModel>> {
        let list = CollectTypeEntity::find()
            .filter(CollectTypeColumn::Id.is_in(collect_type_ids))
            .all(self.conn)
            .await?;
        Ok(list)
    }

    /// 更新视频数
    pub async fn increment_vod(&self, id: Uuid, count: i32) -> ServerResult<()> {
        CollectTypeEntity::update_many()
            .col_expr(CollectTypeColumn::VodCount, Expr::col(CollectTypeColumn::VodCount).add(count))
            .filter(CollectTypeColumn::Id.eq(id))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID修改
    pub async fn update_by_id(&self, record: CollectTypeActiveModel) -> ServerResult<()> {
        CollectTypeEntity::update(record)
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID修改
    pub async fn update_show(&self, id: Uuid, show: bool) -> ServerResult<()> {
        let update = CollectTypeActiveModel {
            id: Set(id),
            show: Set(show),
            ..Default::default()
        };
        CollectTypeEntity::update(update).exec(self.conn).await?;
        Ok(())
    }

}
