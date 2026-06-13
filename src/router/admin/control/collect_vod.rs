use crate::common::handler::AdminHandler;
use crate::common::model::Pager;
use crate::common::Response;
use crate::router::admin::service::collect_vod::CollectVodService;
use crate::router::admin::vo::collect_vod::{CollectEpisodeReq, CollectLineResp, CollectVodListReq, CollectVodListResp};
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/collect/vod")
            .service(list)
            .service(episode)
    );
}

#[post("/list")]
async fn list(req: HttpRequest, param: Json<CollectVodListReq>) -> Response<Pager<CollectVodListResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            let resp = CollectVodService::new(state).list(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

#[post("/episode")]
async fn episode(req: HttpRequest, param: Json<CollectEpisodeReq>) -> Response<Vec<CollectLineResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            let resp = CollectVodService::new(state).episode(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

