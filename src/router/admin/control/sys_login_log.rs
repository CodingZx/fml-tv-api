use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::{IdsReq, Pager};
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::vo::sys_login_log::{SysLoginLogListReq, SysLoginLogListResp};
use crate::router::admin::service::sys_login_log::SysLoginLogService;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/sys/login-log")
            .service(list)
            .service(delete)
            .service(clear)
    );
}

/// 加载列表
#[post("/list")]
async fn list(req: HttpRequest, param: Json<SysLoginLogListReq>) -> Response<Pager<SysLoginLogListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取登录失败日志列表"))
        .action(async |_, state, param| {
            let pager = SysLoginLogService::new(state).list(param).await?;
            Ok(Some(pager))
        })
        .run()
        .await
}

/// 删除
#[post("/delete")]
async fn delete(req: HttpRequest, param: Json<IdsReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::DELETE("删除登录失败日志"))
        .locked_str_key("sys:login-log:operate")
        .action(async |_, state, param| {
            SysLoginLogService::new(state).delete(param.id).await?;
            Ok(None)
        })
        .run()
        .await
}
/// 清空
#[post("/clear")]
async fn clear(req: HttpRequest) -> Response<Void> {
    AdminHandler::of(req, ())
        .log_info(BisType::OTHER("清空登录失败日志"))
        .locked_str_key("sys:login-log:operate")
        .action(async |_, state, _| {
            SysLoginLogService::new(state).clear().await?;
            Ok(None)
        })
        .run()
        .await
}
