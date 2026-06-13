use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::{IdsReq, Pager};
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::vo::sys_queue_msg::{SysQueueMsgListReq, SysQueueMsgListResp};
use crate::router::admin::service::sys_queue_msg::SysQueueMsgService;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/sys/queue-msg")
            .service(list)
            .service(resend)
            .service(delete)
    );
}

/// 加载列表
#[post("/list")]
async fn list(req: HttpRequest, param: Json<SysQueueMsgListReq>) -> Response<Pager<SysQueueMsgListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取消息列表"))
        .action(async |_, state, param| {
            let pager = SysQueueMsgService::new(state).list(param).await?;
            Ok(Some(pager))
        })
        .run()
        .await
}

/// 重发
#[post("/resend")]
async fn resend(req: HttpRequest, param: Json<IdsReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::OTHER("重发消息"))
        .locked_str_key("lock:rabbit:operate")
        .action(async |_, state, param| {
            SysQueueMsgService::new(state).resend(param.id).await?;
            Ok(None)
        })
        .run()
        .await
}

/// 删除
#[post("/delete")]
async fn delete(req: HttpRequest, param: Json<IdsReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::DELETE("删除消息"))
        .locked_str_key("lock:rabbit:operate")
        .action(async |_, state, param| {
            SysQueueMsgService::new(state).delete(param.id).await?;
            Ok(None)
        })
        .run()
        .await
}
