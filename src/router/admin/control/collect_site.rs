use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::Pager;
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::service::collect_site::CollectSiteService;
use crate::router::admin::vo::collect_site::{CollectSiteCollectReq, CollectSiteDeleteReq, CollectSiteFullCollectReq, CollectSiteListReq, CollectSiteListResp, CollectSiteSaveReq, CollectSiteStatusReq};
use crate::router::admin::vo::ComBoxResp;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/collect/site")
            .service(all)
            .service(list)
            .service(save)
            .service(send_full_collect)
            .service(send_collect)
            .service(update_status)
            .service(delete)
    );
}

#[post("/all")]
async fn all(req: HttpRequest) -> Response<Vec<ComBoxResp>> {
    AdminHandler::of(req, ())
        .action(async |_, state, _| {
            let resp = CollectSiteService::new(state).all().await?;
            Ok(Some(resp))
        })
        .run()
        .await
}


#[post("/list")]
async fn list(req: HttpRequest, param: Json<CollectSiteListReq>) -> Response<Pager<CollectSiteListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取采集站列表"))
        .action(async |_, state, param| {
            let resp = CollectSiteService::new(state).list(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

#[post("/save")]
async fn save(req: HttpRequest, param: Json<CollectSiteSaveReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SAVE("保存采集站信息"))
        .locked_str_key("lock:collect:site:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            CollectSiteService::new(state).save(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

#[post("/send/full/collect")]
async fn send_full_collect(req: HttpRequest, body: Json<CollectSiteFullCollectReq>) -> Response<Void> {
    AdminHandler::of(req, body.into_inner())
        .log_info(BisType::OTHER("发布全量采集任务"))
        .locked_str_key("lock:collect:site:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            CollectSiteService::new(state).send_full_collect(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

#[post("/send/collect")]
async fn send_collect(req: HttpRequest, body: Json<CollectSiteCollectReq>) -> Response<Void> {
    AdminHandler::of(req, body.into_inner())
        .log_info(BisType::OTHER("发布增量采集任务"))
        .locked_str_key("lock:collect:site:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            CollectSiteService::new(state).send_collect(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

#[post("/update/status")]
async fn update_status(req: HttpRequest, body: Json<CollectSiteStatusReq>) -> Response<Void> {
    AdminHandler::of(req, body.into_inner())
        .log_info(BisType::OTHER("修改采集站状态"))
        .locked_str_key("lock:collect:site:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            CollectSiteService::new(state).update_status(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

#[post("/delete")]
async fn delete(req: HttpRequest, param: Json<CollectSiteDeleteReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::DELETE("删除采集站"))
        .locked_str_key("lock:collect:site:operate")
        .action(async |ctx, state, param| {
            let token = ctx.token_or_err()?;
            CollectSiteService::new(state).delete(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}
