use serde::{Deserialize, Serialize};

/// 登录请求内容
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginReq {
    pub username: String, // 账号
    pub password: String,  // 密码
}

/// 登录返回内容
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResp {
    pub access_token: String,
    pub refresh_token: String,
}

/// 修改密码
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePwdReq {
    pub old_pwd: String, // 原密码
    pub new_pwd: String, // 新密码
}
