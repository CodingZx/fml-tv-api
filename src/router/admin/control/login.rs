use crate::common::handler::AdminHandler;
use crate::common::result::Void;
use crate::common::util::ip_util::IPUtil;
use crate::common::Response;
use crate::router::admin::vo::login::{LoginReq, LoginResp, UpdatePwdReq};
use crate::router::admin::service::login::LoginService;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(refresh);
    cfg.service(update_pwd);
    cfg.service(logout);
}

/// 登录
#[post("/admin/login")]
async fn login(req: HttpRequest, param: Json<LoginReq>) -> Response<LoginResp> {
    let req_ip = IPUtil::get_ip(&req);

    AdminHandler::of(req, param.into_inner())
        .skip_login()
        .locked_key(|_| format!("lock:login:{}", req_ip))
        .action(async |_, state, param| {
            let resp = LoginService::new(state).login(param, req_ip).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

/// 刷新 Token
#[post("/admin/token/refresh")]
async fn refresh(req: HttpRequest) -> Response<LoginResp> {
    AdminHandler::of(req, ())
        .use_refresh_token()
        .action(async |ctx, state, _| {
            let token = ctx.token_or_err()?;
            let resp = LoginService::new(state).refresh(token).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}


/// 修改密码
#[post("/admin/update/pwd")]
async fn update_pwd(req: HttpRequest, param: Json<UpdatePwdReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .locked_str_key("lock:update:pwd")
        .lock_user()
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            LoginService::new(state).update_pwd(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

/// 退出
#[post("/admin/logout")]
async fn logout(req: HttpRequest) -> Response<Void> {
    AdminHandler::of(req, ())
        .skip_login()
        .action(async |ctx, state, _| {
            let token = match ctx.token_or_none() {
                None => return Ok(None),
                Some(v) => v,
            };

            LoginService::new(state).logout(token).await?;
            Ok(None)
        })
        .run()
        .await
}
