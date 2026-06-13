use crate::common::ServerResult;
use crate::database::model::sys_config::{SysConfigActiveModel, SysConfigColumn, SysConfigEntity};
use sea_orm::sea_query::OnConflict;
use sea_orm::ColumnTrait;
use sea_orm::{ConnectionTrait, EntityTrait, QueryFilter, Set};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

pub struct SysConfigDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysConfigDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    pub async fn save_conf(&self, key: &str, value: &str) -> ServerResult<()> {
        let model = SysConfigActiveModel {
            id: Set(Uuid::now_v7()),
            key: Set(key.to_string()),
            conf_value: Set(value.to_string()),
        };
        SysConfigEntity::insert(model)
            .on_conflict(
                OnConflict::column(SysConfigColumn::Key)
                    .value(SysConfigColumn::ConfValue, value.to_string())
                    .to_owned()
            )
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }
    
    pub async fn save_json_conf<T: Serialize>(&self, key: &str, value: &T) -> ServerResult<()> {
        let value = serde_json::to_string(value)?;
        self.save_conf(key, &value).await
    }
    
    pub async fn get_json_conf<T: DeserializeOwned>(&self, key: &str) -> ServerResult<Option<T>> {
        let str = self.get_str_conf(key, "").await?;
        if str.is_empty() { 
            return Ok(None);
        }
        let value = serde_json::from_str::<T>(str.as_str())?;
        Ok(Some(value))
    }

    pub async fn get_str_conf(&self, key: &str, default: &str) -> ServerResult<String> {
        let value = self.get_conf(key).await?.unwrap_or(default.to_string());
        Ok(value)
    }

    async fn get_conf(&self, key: &str) -> ServerResult<Option<String>> {
        let model = SysConfigEntity::find()
            .filter(SysConfigColumn::Key.eq(key))
            .one(self.conn)
            .await?;
        Ok(model.map(|v| v.conf_value))
    }
}
