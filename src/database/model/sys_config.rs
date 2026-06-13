use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};

pub type SysConfigActiveModel = ActiveModel;
pub type SysConfigModel = Model;
pub type SysConfigEntity = Entity;
pub type SysConfigColumn = Column;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: Uuid,
    #[sea_orm(column_name = "key")]
    pub key: String,
    #[sea_orm(column_name = "conf_value")]
    pub conf_value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EasyBangumiConfig {
    pub request_url: String,
    pub request_token: String,

    pub js_enable: bool,
    pub js_key: String,
    pub js_label: String,
    pub js_cover: String,
    pub js_version_name: String,
    pub js_version_code: String,
    pub js_lib_ver: String,
}

impl Default for EasyBangumiConfig {
    fn default() -> Self {
        Self {
            request_url: "".to_string(),
            request_token: "".to_string(),
            js_enable: false,
            js_key: "cc.fml".to_string(),
            js_label: "fml-tv".to_string(),
            js_cover: "http://video.yunstou.com/upload/2026-06-11/019eb6ce-fc59-7412-bb08-2f2b992f33d9.webp".to_string(),
            js_version_name: "1.0.0".to_string(),
            js_version_code: "1".to_string(),
            js_lib_ver: "13".to_string(),
        }
    }
}