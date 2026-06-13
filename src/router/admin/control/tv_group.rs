use crate::common::handler::AdminHandler;
use crate::common::log_info::BisType;
use crate::common::model::{IdsReq, Pager};
use crate::common::result::Void;
use crate::common::Response;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest};
use crate::router::admin::service::tv_group::TvGroupService;
use crate::router::admin::vo::tv_group::{TvGroupListReq, TvGroupListResp, TvGroupSaveReq};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/tv/group")
            .service(list)
            .service(save)
            .service(delete),
    );
}


#[post("/list")]
async fn list(req: HttpRequest, param: Json<TvGroupListReq>) -> Response<Pager<TvGroupListResp>> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SELECT("获取视频类型列表"))
        .action(async |_, state, param| {
            let pager = TvGroupService::new(state).list(param).await?;
            Ok(Some(pager))
        })
        .run()
        .await
}

#[post("/save")]
async fn save(req: HttpRequest, param: Json<TvGroupSaveReq>) -> Response<Void> {
    AdminHandler::of(req, param.into_inner())
        .log_info(BisType::SAVE("保存视频类型"))
        .locked_str_key("lock:tv:type:operate")
        .action(async |ctx, state, param| {
            param.validate()?;

            let token = ctx.token_or_err()?;
            TvGroupService::new(state).save(token, param).await?;
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
            TvGroupService::new(state).delete(param).await?;
            Ok(None)
        })
        .run()
        .await
}
