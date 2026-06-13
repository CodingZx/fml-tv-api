use crate::common::model::Page;
use crate::common::{consts, ServerResult};
use crate::database::model::collect_vod::{CollectVodActiveModel, CollectVodColumn, CollectVodEntity, CollectVodModel};
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter};
use sea_orm::{DbBackend, FromQueryResult, PaginatorTrait, QueryOrder, QuerySelect, Statement};
use uuid::Uuid;

pub struct CollectVodDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> CollectVodDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }


    /// 查询列表
    pub async fn find_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<CollectVodModel>, u64)>{
        let total = CollectVodEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = CollectVodEntity::find()
            .filter(condition)
            .order_by_desc(CollectVodColumn::CreateTime)
            .order_by_desc(CollectVodColumn::Id)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }

    pub async fn find_site_by_id(&self, id: Uuid) -> ServerResult<Uuid> {
        let sql = r#"select site_id from collect_vod where id = $1"#;

        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            vec![sea_orm::Value::Uuid(Some(id))],
        );
        let site_id = VodSite::find_by_statement(statement).one(self.conn).await?.map(|t|t.site_id).unwrap_or(consts::get_default_id());
        Ok(site_id)
    }

    /// 查询采集视频
    pub async fn find_by_vod_id(&self, site_id: Uuid, vod_id: &str) -> ServerResult<Option<CollectVodModel>> {
        let model = CollectVodEntity::find()
            .filter(CollectVodColumn::SiteId.eq(site_id))
            .filter(CollectVodColumn::VodId.eq(vod_id))
            .one(self.conn)
            .await?;
        Ok(model)
    }

    /// Insert
    pub async fn insert(&self, model: CollectVodActiveModel) -> ServerResult<()> {
        CollectVodEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID修改
    pub async fn update_by_id(&self, record: CollectVodActiveModel) -> ServerResult<()> {
        CollectVodEntity::update(record)
            .exec(self.conn)
            .await?;
        Ok(())
    }
}

#[derive(Debug, FromQueryResult)]
pub struct VodSite {
    pub site_id: Uuid,
}