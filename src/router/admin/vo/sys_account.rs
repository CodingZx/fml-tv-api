use crate::common::consts::system::{ADMIN_ACCOUNT_MAX_LEN, ADMIN_ACCOUNT_MIN_LEN};
use crate::common::error::ServerError::{BusinessError, BusinessStrError};
use crate::common::util::time_util::TimeUtil;
use crate::common::ServerResult;
use crate::database::model::sys_account::SysAccountModel;
use crate::router::admin::vo::VersionIdReq;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 查询列表-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysAccountListReq {
    pub page: u64,
    pub size: u64,

    pub real_name: Option<String>,
    pub user_name: Option<String>,
}

/// 保存账号-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysAccountSaveReq {
    pub id: Option<Uuid>,
    pub username: String,
    pub real_name: String,
    pub password: Option<String>,
    pub version: i32,
}

impl SysAccountSaveReq {
    pub fn validate(&self) -> ServerResult<()> {
        if self.username.is_empty() {
            return Err(BusinessError("账号不能为空"));
        }
        let user_name_len = self.username.chars().count();
        if user_name_len < ADMIN_ACCOUNT_MIN_LEN {
            return Err(BusinessStrError(format!("账号长度不能小于{}", ADMIN_ACCOUNT_MIN_LEN)));
        }
        if user_name_len > ADMIN_ACCOUNT_MAX_LEN {
            return Err(BusinessStrError(format!("账号长度不能大于{}", ADMIN_ACCOUNT_MAX_LEN)));
        }
        if self.real_name.is_empty() {
            return Err(BusinessError("真实姓名不能为空"));
        }
        if self.id.is_none() {
            // 新增
            if self.password.is_none() {
                return Err(BusinessError("密码不能为空"));
            }
        }
        Ok(())
    }
}

/// 重置密码-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysAccountResetPwdReq {
    pub id: Uuid,
    pub password: String,
    pub version: i32,
}

/// 修改状态-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysAccountStatusReq {
    pub id: Uuid,
    pub status: bool,
    pub version: i32,
}

/// 列表信息-返回
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysAccountListResp {
    pub id: Uuid,          // 记录ID
    pub username: String, // 用户名
    pub real_name: String, // 真实姓名
    pub status: bool,      // 状态

    pub update_time: String, // 修改时间
    pub version: i32,        // 版本号
}

impl SysAccountListResp {
    pub fn new(account: SysAccountModel) -> Self {
        Self {
            id: account.id,
            username: account.username,
            real_name: account.real_name,
            status: account.status,
            update_time: TimeUtil::format_default(account.update_time),
            version: account.version,
        }
    }
}

/// 删除信息-请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysAccountDeleteReq {
    pub id_ver: Vec<VersionIdReq>,
    pub reason: String,
}