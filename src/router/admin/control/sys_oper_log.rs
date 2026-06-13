use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::{IdReq, IdsReq, Pager};
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::vo::sys_oper_log::{SysOperLogDetailResp, SysOperLogListReq, SysOperLogListResp};
use crate::router::admin::service::sys_oper_log::SysOperLogService;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/sys/oper-log")
            .service(list)
            .service(clear)
            .service(delete)
            .service(detail),
    );
}

/// 查询列表
#[post("/list")]
async fn list(req: HttpRequest, param: Json<SysOperLogListReq>) -> Response<Pager<SysOperLogListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取日志列表"))
        .action(async |_, state, param| {
            let pager = SysOperLogService::new(state).list(param).await?;

            Ok(Some(pager))
        })
        .run()
        .await
}

#[post("/detail")]
async fn detail(req: HttpRequest, param: Json<IdReq>) -> Response<SysOperLogDetailResp> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取日志详情"))
        .action(async |_, state, param| {
            let detail = SysOperLogService::new(state).detail(param.id).await?;
            Ok(Some(detail))
        })
        .run()
        .await
}

/// 删除
#[post("/delete")]
async fn delete(req: HttpRequest, param: Json<IdsReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::DELETE("删除日志"))
        .locked_str_key("lock:oper-log:operate")
        .action(async |_, state, param| {
            SysOperLogService::new(state).delete(param.id).await?;
            Ok(None)
        })
        .run()
        .await
}

/// 清空
#[post("/clear")]
async fn clear(req: HttpRequest) -> Response<Void> {
    AdminHandler::of(req, ())
        .log_info(BisType::OTHER("清空日志"))
        .locked_str_key("lock:oper-log:operate")
        .action(async |_, state, _| {
            SysOperLogService::new(state).clear().await?;
            Ok(None)
        })
        .run()
        .await
}
