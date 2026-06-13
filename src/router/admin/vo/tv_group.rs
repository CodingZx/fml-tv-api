use crate::common::error::ServerError::BusinessError;
use crate::common::util::time_util::TimeUtil;
use crate::common::ServerResult;
use crate::database::model::tv_group::TvGroupModel;
use crate::database::model::tv_type::TvTypeModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvGroupListReq {
    pub page: u64,
    pub size: u64,

    pub name: Option<String>,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TvGroupListResp {
    pub id: Uuid,            // 记录ID
    pub name: String,        // 名称
    pub sort_num: i32, // 排序值
    pub update_time: String, // 修改时间
    pub version: i32,        // 版本号
    pub types: Vec<String>,
    pub type_ids: Vec<Uuid>,
}

impl TvGroupListResp {
    pub fn new(model: TvGroupModel, types: Vec<TvTypeModel>) -> Self {
        let mut type_names = Vec::new();
        let mut type_ids = Vec::new();
        for type_model in types {
            type_names.push(type_model.name);
            type_ids.push(type_model.id);
        }
        Self {
            id: model.id,
            name: model.name,
            update_time: TimeUtil::format_default(model.update_time),
            version: model.version,
            types: type_names,
            type_ids,
            sort_num: model.sort_num,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvGroupSaveReq {
    pub id: Option<Uuid>,
    pub name: String,
    pub version: i32,
    pub type_ids: Vec<Uuid>,
    pub sort_num: i32,
}
impl TvGroupSaveReq {
    pub fn validate(&self) -> ServerResult<()> {
        if self.name.is_empty() {
            return Err(BusinessError("名称不能为空"))
        }
        if self.type_ids.is_empty() { 
            return Err(BusinessError("请选择绑定的视频分类"))
        }
        Ok(())
    }
}

