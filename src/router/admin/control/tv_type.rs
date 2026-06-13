use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::{IdsReq, Pager};
use crate::common::result::Void;
use crate::common::Response;
use crate::router::admin::service::tv_type::TvTypeService;
use crate::router::admin::vo::tv_type::{TvTypeListReq, TvTypeListResp, TvTypeSaveReq};
use crate::router::admin::vo::ComBoxResp;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/tv/type")
            .service(all)
            .service(list)
            .service(save)
            .service(delete),
    );
}

#[post("/all")]
async fn all(req: HttpRequest) -> Response<Vec<ComBoxResp>> {
    AdminHandler::of(req, ())
        .action(async |_, state, _| {
            let pager = TvTypeService::new(state).all().await?;
            Ok(Some(pager))
        })
        .run()
        .await
}


#[post("/list")]
async fn list(req: HttpRequest, param: Json<TvTypeListReq>) -> Response<Pager<TvTypeListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取视频类型列表"))
        .action(async |_, state, param| {
            let pager = TvTypeService::new(state).list(param).await?;
            Ok(Some(pager))
        })
        .run()
        .await
}

#[post("/save")]
async fn save(req: HttpRequest, param: Json<TvTypeSaveReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SAVE("保存视频类型"))
        .locked_str_key("lock:tv:type:operate")
        .action(async |ctx, state, param| {
            param.validate()?;

            let token = ctx.token_or_err()?;
            TvTypeService::new(state).save(token, param).await?;
            Ok(None)
        })
        .run()
        .await
}

#[post("/delete")]
async fn delete(req: HttpRequest, param: Json<IdsReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::DELETE("删除视频类型"))
        .locked_str_key("lock:tv:type:operate")
        .action(async |_, state, param| {
            TvTypeService::new(state).delete(param).await?;
            Ok(None)
        })
        .run()
        .await
}
