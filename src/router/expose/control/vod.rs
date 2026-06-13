use crate::common::error::ServerError::BusinessError;
use crate::common::model::ExposePager;
use crate::common::result::ResultData;
use crate::common::state::AppState;
use crate::common::{consts, Response, ServerResult};
use crate::database::dao::sys_config::SysConfigDao;
use crate::database::model::sys_config::EasyBangumiConfig;
use crate::router::expose::service::vod::VodService;
use crate::router::expose::vo::vod::{JsQuery, TypeGroupReq, TypeListResp, VodDetailReq, VodDetailResp, VodEpisodePlayReq, VodEpisodePlayResp, VodListReq, VodListResp, VodSearchReq};
use actix_web::http::header;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/expose")
            .service(js_content)
            .service(types)
            .service(vod_list)
            .service(vod_detail)
            .service(vod_search)
            .service(vod_play_url)
    );
}

/// 获取视频
#[get("/fml.js")]
async fn js_content(state: Data<AppState>, param: Query<JsQuery>) -> impl Responder {
    let js_content = match VodService::new(&state).js_content(param.into_inner()).await {
        Ok(content) => content,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript; charset=utf-8"))
        .insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        .body(js_content)
}

/// 获取视频
#[post("/types")]
async fn types(state: Data<AppState>, param: Json<TypeGroupReq>) -> Response<Vec<TypeListResp>> {
    // 验证令牌
    check_req_token(&state, &param.req_token).await?;

    let resp = VodService::new(&state).types(param.into_inner()).await?;
    ResultData::success_data(resp)
}

/// 获取视频列表
#[post("/list")]
async fn vod_list(state: Data<AppState>, param: Json<VodListReq>) -> Response<ExposePager<VodListResp>> {
    // 验证令牌
    check_req_token(&state, &param.req_token).await?;

    let resp = VodService::new(&state).vod_list(param.into_inner()).await?;
    ResultData::success_data(resp)
}

/// 获取视频详情
#[post("/detail")]
async fn vod_detail(state: Data<AppState>, param: Json<VodDetailReq>) -> Response<VodDetailResp> {
    // 验证令牌
    check_req_token(&state, &param.req_token).await?;

    let resp = VodService::new(&state).vod_detail(param.into_inner()).await?;
    ResultData::success_data(resp)
}

/// 搜索视频
#[post("/search")]
async fn vod_search(state: Data<AppState>, param: Json<VodSearchReq>) -> Response<ExposePager<VodListResp>> {
    // 验证令牌
    check_req_token(&state, &param.req_token).await?;

    let resp = VodService::new(&state).vod_search(param.into_inner()).await?;
    ResultData::success_data(resp)
}

/// 播放地址
#[post("/play/url")]
async fn vod_play_url(state: Data<AppState>, param: Json<VodEpisodePlayReq>) -> Response<VodEpisodePlayResp> {
    // 验证令牌
    check_req_token(&state, &param.req_token).await?;

    let resp = VodService::new(&state).play_url(param.into_inner()).await?;
    ResultData::success_data(resp)
}


async fn check_req_token(state: &AppState, req_token: &str) -> ServerResult<()> {
    let conf_key = consts::conf_keys::easy_bangumi_conf_key();
    let config = SysConfigDao::new(&state.db).get_json_conf::<EasyBangumiConfig>(&conf_key).await?.unwrap_or_default();
    if config.request_token != req_token {
        return Err(BusinessError("请求令牌错误"))
    }
    Ok(())
}
