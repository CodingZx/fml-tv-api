use crate::common::state::AppState;
use crate::common::{consts, ServerResult};
use crate::database::dao::sys_config::SysConfigDao;
use crate::database::model::sys_config::EasyBangumiConfig;
use crate::router::admin::vo::sys_config::{SysConfigSaveReq, SysConfigSaveResp};
use sea_orm::TransactionTrait;
use std::sync::Arc;

pub struct SysConfigService {
    state: Arc<AppState>,
}

impl SysConfigService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn config(&self) -> ServerResult<SysConfigSaveResp> {
        let dao = SysConfigDao::new(&self.state.db);

        let easy_bangumi_conf_key =  consts::conf_keys::easy_bangumi_conf_key();
        let easy_bangumi_conf = dao.get_json_conf::<EasyBangumiConfig>(&easy_bangumi_conf_key).await?.unwrap_or_default();

        let resp = SysConfigSaveResp {
            request_token: easy_bangumi_conf.request_token,
            request_url: easy_bangumi_conf.request_url,
            js_enable: easy_bangumi_conf.js_enable,
            js_key: easy_bangumi_conf.js_key,
            js_label: easy_bangumi_conf.js_label,
            js_cover: easy_bangumi_conf.js_cover,
            js_version_name: easy_bangumi_conf.js_version_name,
            js_version_code: easy_bangumi_conf.js_version_code,
            js_lib_ver: easy_bangumi_conf.js_lib_ver,
        };

        Ok(resp)
    }

    /// 保存
    pub async fn save(&self, mut param: SysConfigSaveReq) -> ServerResult<()> {
        if param.request_url.ends_with("/") {
            param.request_url = param.request_url[..param.request_url.len() - 1].to_string();
        }

        let config = EasyBangumiConfig {
            request_url: param.request_url,
            request_token: param.request_token,
            js_enable: param.js_enable,
            js_key: param.js_key,
            js_label: param.js_label,
            js_cover: param.js_cover,
            js_version_name: param.js_version_name,
            js_version_code: param.js_version_code,
            js_lib_ver: param.js_lib_ver,
        };

        self.state.db.transaction(|tx| {
            Box::pin(async move {
                let dao = SysConfigDao::new(tx);

                let easy_bangumi_conf_key = consts::conf_keys::easy_bangumi_conf_key();
                dao.save_json_conf(&easy_bangumi_conf_key, &config).await?;

                Ok(())
            })
        }).await?;
        Ok(())
    }

}
