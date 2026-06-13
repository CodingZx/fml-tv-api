use actix_web::{get, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde_json::json;

mod admin;
mod expose;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(health);
    admin::control::register(cfg);
    expose::control::register(cfg);
}

#[get("/health")]
async fn health() -> impl Responder {
    let json = json!({
        "status": "ok"
    });
    HttpResponse::Ok().json(json)
}
