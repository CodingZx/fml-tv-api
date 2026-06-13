use crate::common::ServerResult;
use crate::database::model::tv_type_bind::{TvTypeBindActiveModel, TvTypeBindColumn, TvTypeBindEntity, TvTypeBindModel};
use sea_orm::{ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};
use uuid::Uuid;

pub struct TvTypeBindDao<'d, C: ConnectionTrait> {
    conn: &'d C
}

impl<'d, C: ConnectionTrait> TvTypeBindDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 批量写入
    pub async fn batch_insert(&self, binds: Vec<TvTypeBindActiveModel>) -> ServerResult<()> {
        if !binds.is_empty() {
            TvTypeBindEntity::insert_many(binds)
                .exec_without_returning(self.conn)
                .await?;
        }
        Ok(())
    }

    pub async fn find_by_site_id(&self, site_id: Uuid) -> ServerResult<Vec<TvTypeBindModel>> {
        let binds = TvTypeBindEntity::find()
            .filter(TvTypeBindColumn::SiteId.eq(site_id))
            .all(self.conn)
            .await?;
        Ok(binds)
    }

    pub async fn delete_by_site_id(&self, site_id: Uuid) -> ServerResult<()> {
        TvTypeBindEntity::delete_many()
            .filter(TvTypeBindColumn::SiteId.eq(site_id))
            .exec(self.conn)
            .await?;
        Ok(())
    }
    
    pub async fn delete_by_collect_type_id(&self, collect_type_id: Uuid) -> ServerResult<()> {
        TvTypeBindEntity::delete_many()
            .filter(TvTypeBindColumn::CollectTypeId.eq(collect_type_id))
            .exec(self.conn)
            .await?;
        Ok(())
    }

    pub async fn find_by_collect_type(&self, collect_type_id: Uuid) -> ServerResult<Vec<TvTypeBindModel>> {
        let binds = TvTypeBindEntity::find()
            .filter(TvTypeBindColumn::CollectTypeId.eq(collect_type_id))
            .all(self.conn)
            .await?;
        Ok(binds)
    }

    pub async fn find_by_collect_types(&self, collect_types: impl IntoIterator<Item = Uuid>) -> ServerResult<Vec<TvTypeBindModel>> {
        let binds = TvTypeBindEntity::find()
            .filter(TvTypeBindColumn::CollectTypeId.is_in(collect_types))
            .all(self.conn)
            .await?;
        Ok(binds)
    }

    pub async fn find_by_tv_type(&self, tv_type_id: Uuid) -> ServerResult<Vec<TvTypeBindModel>> {
        let binds = TvTypeBindEntity::find()
            .filter(TvTypeBindColumn::TvTypeId.eq(tv_type_id))
            .all(self.conn)
            .await?;
        Ok(binds)
    }

    pub async fn find_x18_by_tv_type(&self, tv_type_id: Uuid) -> ServerResult<bool> {
        let sql = r#" select count(ct.id) as x18_count from tv_type_bind tb
                    inner join collect_type ct on ct.id = tb.collect_type_id
                    where tb.tv_type_id = $1
                    and ct.x18 = true "#;
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            vec![sea_orm::Value::Uuid(Some(tv_type_id))],
        );
        let count = HasX18::find_by_statement(statement).one(self.conn).await?.map(|t|t.x18_count).unwrap_or(0);
        Ok(count > 0)
    }

}


#[derive(Debug, FromQueryResult)]
struct HasX18 {
    x18_count: i32,
}
