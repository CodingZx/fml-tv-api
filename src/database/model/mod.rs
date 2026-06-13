use std::collections::HashSet;
use sea_orm::{DeriveActiveEnum, EnumIter, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod sys_account;
pub mod sys_config;
pub mod sys_oper_log;
pub mod sys_login_log;
pub mod collect_site;
pub mod collect_type;
pub mod collect_vod;
pub mod collect_vod_episode;
pub mod tv_group;
pub mod tv_type;
pub mod tv_type_bind;
pub mod tv_vod;
pub mod tv_vod_pic;

/// 操作日志 业务类型
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum BusinessTypes {
    #[sea_orm(num_value = 0)]
    Other,
    #[sea_orm(num_value = 1)]
    Save,
    #[sea_orm(num_value = 2)]
    Select,
    #[sea_orm(num_value = 3)]
    Delete,
}

impl BusinessTypes {
    pub fn get_name(&self) -> String {
        match self {
            BusinessTypes::Other => String::from("其他"),
            BusinessTypes::Save => String::from("保存"),
            BusinessTypes::Select => String::from("查询"),
            BusinessTypes::Delete => String::from("删除"),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct HashUUIDs(pub HashSet<Uuid>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct VecUUIDs(pub Vec<Uuid>);
