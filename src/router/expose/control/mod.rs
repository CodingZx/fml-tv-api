use actix_web::web::ServiceConfig;

mod vod;

/// 注册 Router
pub fn register(cfg: &mut ServiceConfig) {
    vod::register(cfg);
}