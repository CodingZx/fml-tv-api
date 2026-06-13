extern crate core;

use crate::common::cron::CornManager;
use crate::common::logger::Level;
use crate::common::pgq::consumer::ConsumerRegister;
use crate::common::state::AppState;
use crate::common::{logger, pgcache, pglock};
use actix_cors::Cors;
use actix_http::KeepAlive;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use std::env;
use std::sync::Arc;
use std::time::Duration;

pub mod common;
pub mod consumer;
pub mod database;
pub mod router;
pub mod scheduler;

#[actix_web::main]
async fn main() {
    logger::init();

    let mut profile = String::from("dev");
    if let Ok(env_profile) = env::var("APP_PROFILE") {
        profile = env_profile;
    }
    logger::info!("当前启动配置环境:{profile}");

    let conf = common::conf::read_conf(format!("conf/app-{profile}.yml"));
    let state = Arc::new(AppState::new(&conf).await);

    init_consumer(Arc::clone(&state)).await;
    init_scheduler(Arc::clone(&state)).await;

    // 绑定端口
    let bind_port = conf.server.port;
    // 启动HttpServer
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default().log_level(Level::Debug))
            .wrap(middleware::Compress::default())
            .app_data(conf.server.json_config())
            .app_data(Data::from(Arc::clone(&state)))
            .configure(router::routes)
    })
        .keep_alive(KeepAlive::Timeout(Duration::from_secs(60)))
        .bind(("0.0.0.0", bind_port))
        .expect("无法创建Web Server")
        .run()
        .await
        .expect("Web Server 启动失败");
}

/// 初始化消费者
async fn init_consumer(state: Arc<AppState>) {
    let mut register = ConsumerRegister::new(Arc::clone(&state));
    consumer::register_consumer(&mut register, Arc::clone(&state));
    register.start().await.expect("启动Consumer时发生错误");
}

/// 初始化定时任务
async fn init_scheduler(state: Arc<AppState>) {
    let mut ops = CornManager::new().await;
    // 调用 Corn 注册的定时任务
    scheduler::register(&mut ops, Arc::clone(&state)).await;
    pglock::register(&mut ops, Arc::clone(&state)).await;
    pgcache::register(&mut ops, Arc::clone(&state)).await;
    // 启动定时任务
    ops.start().await;
}
