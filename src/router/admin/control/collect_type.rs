use crate::common::handler::AdminHandler;
use crate::common::model::IdReq;
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::service::collect_type::CollectTypeService;
use crate::router::admin::vo::collect_type::{CollectTypeResp, CollectTypeSaveBindReq, CollectTypeStatusReq};
use crate::router::admin::vo::ComBoxResp;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/collect/type")
            .service(all)
            .service(list)
            .service(bind)
            .service(status)
    );
}

#[post("/all")]
async fn all(req: HttpRequest, param: Json<IdReq>) -> Response<Vec<ComBoxResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            let resp = CollectTypeService::new(state).all(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}
#[post("/list")]
async fn list(req: HttpRequest, param: Json<IdReq>) -> Response<Vec<CollectTypeResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            let resp = CollectTypeService::new(state).site_types(param).await?;
            Ok(Some(resp))
        })
        .run()
        .await
}

#[post("/bind")]
async fn bind(req: HttpRequest, param: Json<CollectTypeSaveBindReq>) -> Response<Vec<CollectTypeResp>> {
    AdminHandler::of(req, param.into_inner())
        .action(async |_, state, param| {
            CollectTypeService::new(state).bind_types(param).await?;
            Ok(None)
        })
        .run()
        .await
}

#[post("/status")]
async fn status(req: HttpRequest, param: Json<CollectTypeStatusReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .locked_str_key("lock:collect:type:operate")
        .action(async |_, state, param| {
            CollectTypeService::new(state).update_status(param).await?;
            Ok(None)
        })
        .run()
        .await
}

