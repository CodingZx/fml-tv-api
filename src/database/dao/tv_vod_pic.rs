use crate::common::ServerResult;
use crate::database::model::tv_vod_pic::{TvVodPicActiveModel, TvVodPicColumn, TvVodPicEntity, TvVodPicModel};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

pub struct TvVodPicDao<'d, C: ConnectionTrait> {
    conn: &'d C
}

impl<'d, C: ConnectionTrait> TvVodPicDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 根据TvVodId查询
    pub async fn find_by_vod_id(&self, tv_vod_id: Uuid) -> ServerResult<Vec<TvVodPicModel>> {
        let pics = TvVodPicEntity::find()
            .filter(TvVodPicColumn::TvVodId.eq(tv_vod_id))
            .all(self.conn)
            .await?;
        Ok(pics)
    }
    
    pub async fn find_one_by_vod_id(&self, vod_id: Uuid) -> ServerResult<Option<TvVodPicModel>> {
        let pic = TvVodPicEntity::find()
            .filter(TvVodPicColumn::TvVodId.eq(vod_id))
            .filter(TvVodPicColumn::Status.eq(true))
            .limit(1)
            .one(self.conn)
            .await?;
        Ok(pic)
    }

    pub async fn find_by_collect_vod(&self, tv_vod_id: Uuid, collect_vod_id: Uuid) -> ServerResult<Option<TvVodPicModel>> {
        let pic = TvVodPicEntity::find()
            .filter(TvVodPicColumn::TvVodId.eq(tv_vod_id))
            .filter(TvVodPicColumn::CollectVodId.eq(collect_vod_id))
            .one(self.conn)
            .await?;
        Ok(pic)
    }

    /// 新增
    pub async fn insert(&self, record: TvVodPicActiveModel) -> ServerResult<()> {
        TvVodPicEntity::insert(record)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID修改
    pub async fn update_by_id(&self, record: TvVodPicActiveModel) -> ServerResult<()> {
        TvVodPicEntity::update(record).exec(self.conn).await?;
        Ok(())
    }

}

