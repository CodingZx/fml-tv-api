use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::Pager;
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::vo::sys_account::{SysAccountDeleteReq, SysAccountListReq, SysAccountListResp, SysAccountResetPwdReq, SysAccountSaveReq, SysAccountStatusReq};
use crate::router::admin::service::sys_account::SysAccountService;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/sys/account")
            .service(list)
            .service(save)
            .service(reset_pwd)
            .service(update_status)
            .service(delete),
    );
}

/// 加载列表
#[post("/list")]
async fn list(req: HttpRequest, param: Json<SysAccountListReq>) -> Response<Pager<SysAccountListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取用户列表"))
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            let pager = SysAccountService::new(state).list(token, param).await?;

            Ok(Some(pager))
        })
        .run()
        .await
}

/// 保存账号
#[post("/save")]
async fn save(req: HttpRequest, param: Json<SysAccountSaveReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SAVE("保存用户"))
        .locked_str_key("lock:account:operate")
        .action(async |ctx, state, param| {
            param.validate()?;

            let token = ctx.token_or_err()?;
            SysAccountService::new(state).save(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

/// 修改状态
#[post("/update/status")]
async fn update_status(req: HttpRequest, param: Json<SysAccountStatusReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::OTHER("修改用户状态"))
        .locked_str_key("lock:account:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            SysAccountService::new(state).update_status(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

/// 重置密码
#[post("/reset/pwd")]
async fn reset_pwd(req: HttpRequest, param: Json<SysAccountResetPwdReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::OTHER("重置用户密码"))
        .locked_str_key("lock:account:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            SysAccountService::new(state).reset_pwd(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

/// 删除
#[post("/delete")]
async fn delete(req: HttpRequest, param: Json<SysAccountDeleteReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::DELETE("删除用户"))
        .locked_str_key("lock:account:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            let service = SysAccountService::new(state);
            service.delete(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}
