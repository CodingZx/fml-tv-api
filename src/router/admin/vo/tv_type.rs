use crate::common::error::ServerError::BusinessError;
use crate::common::util::time_util::TimeUtil;
use crate::common::ServerResult;
use crate::database::model::tv_type::TvTypeModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvTypeListReq {
    pub page: u64,
    pub size: u64,

    pub name: Option<String>,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TvTypeListResp {
    pub id: Uuid,            // 记录ID
    pub name: String,        // 名称
    pub sort_num: i32,       // 排序值
    pub update_time: String, // 修改时间
    pub version: i32,        // 版本号
}

impl TvTypeListResp {
    pub fn new(model: TvTypeModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            sort_num: model.sort_num,
            update_time: TimeUtil::format_default(model.update_time),
            version: model.version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvTypeSaveReq {
    pub id: Option<Uuid>,
    pub name: String,
    pub sort_num: i32,
    pub version: i32,
}
impl TvTypeSaveReq {
    pub fn validate(&self) -> ServerResult<()> {
        if self.name.is_empty() {
            return Err(BusinessError("名称不能为空"))
        }
        Ok(())
    }
}

