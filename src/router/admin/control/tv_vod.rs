use crate::common::handler::AdminHandler;
use crate::common::model::{IdReq, Pager};
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::service::tv_vod::TvVodService;
use crate::router::admin::vo::tv_vod::{TvVodLineResp, TvVodListReq, TvVodListResp, TvVodShowReq};
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/tv/vod")
            .service(list)
            .service(episode)
            .service(status)
    );
}

#[post("/list")]
async fn list(req: HttpRequest, param: Json<TvVodListReq>) -> Response<Pager<TvVodListResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            let resp = TvVodService::new(state).list(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

#[post("/episode")]
async fn episode(req: HttpRequest, param: Json<IdReq>) -> Response<Vec<TvVodLineResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            let resp = TvVodService::new(state).episode(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

#[post("/status")]
async fn status(req: HttpRequest, param: Json<TvVodShowReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .locked_str_key("lock:tv:vod:operate")
        .action(async |_, state, param| {
            TvVodService::new(state).update_status(param).await?;
            Ok(None)
        })
        .run()
        .await
}

