use crate::common::ServerResult;
use crate::database::model::collect_vod_episode::{CollectVodEpiActiveModel, CollectVodEpiColumn, CollectVodEpiEntity, CollectVodEpiModel};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

pub struct CollectVodEpiDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> CollectVodEpiDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 查询采集视频
    pub async fn find_by_vod_id(&self, vod_id: Uuid) -> ServerResult<Vec<CollectVodEpiModel>> {
        let model = CollectVodEpiEntity::find()
            .filter(CollectVodEpiColumn::CollectVodId.eq(vod_id))
            .order_by_asc(CollectVodEpiColumn::SortNum)
            .all(self.conn)
            .await?;
        Ok(model)
    }

    /// 查询采集视频
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<CollectVodEpiModel>> {
        let model = CollectVodEpiEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(model)
    }

    /// 批量写入
    pub async fn batch_insert(&self, episodes: Vec<CollectVodEpiActiveModel>) -> ServerResult<()> {
        if !episodes.is_empty() {
            CollectVodEpiEntity::insert_many(episodes)
                .exec_without_returning(self.conn)
                .await?;
        }
        Ok(())
    }

    /// 根据视频ID删除
    pub async fn delete_by_vod_id(&self, vod_id: Uuid) -> ServerResult<()> {
        CollectVodEpiEntity::delete_many()
            .filter(CollectVodEpiColumn::CollectVodId.eq(vod_id))
            .exec(self.conn)
            .await?;
        Ok(())
    }
}
