use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysConfigSaveReq {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysConfigSaveResp {
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