use crate::common::handler::AdminHandler;
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::service::sys_config::SysConfigService;
use crate::router::admin::vo::sys_config::{SysConfigSaveReq, SysConfigSaveResp};
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/sys/config")
            .service(detail)
            .service(save)
    );
}

#[post("/detail")]
async fn detail(req: HttpRequest) -> Response<SysConfigSaveResp> {
    AdminHandler::of(req, ())
        .action(async |_, state, _| {
            let resp = SysConfigService::new(state).config().await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

#[post("/save")]
async fn save(req: HttpRequest, param: Json<SysConfigSaveReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .locked_str_key("lock:sys:config:operate")
        .action(async |_, state, param| {
            SysConfigService::new(state).save(param).await?;
            Ok(None)
        })
        .run()
        .await
}
