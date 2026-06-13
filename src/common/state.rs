use crate::common::conf::ServerConf;
use sea_orm::DbConn;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DbConn,
    pub pgq_conf: PgqConf,
}

impl AppState {

    pub async fn new(conf: &ServerConf) -> Self {
        let db = conf.database.connect().await.expect("无法创建数据库链接, 请确认配置");
        let pgq_conf = PgqConf {
            enable: conf.pgq.enable,
            fetch_interval_ms: conf.pgq.fetch_interval_ms,
            delay_fetch_interval_ms: conf.pgq.delay_fetch_interval_ms,
            auto_clean: conf.pgq.auto_clean,
        };
        Self { db, pgq_conf }
    }
}

#[derive(Debug, Clone)]
pub struct PgqConf {
    pub enable: bool,
    pub fetch_interval_ms: u64,
    pub delay_fetch_interval_ms: u64,
    pub auto_clean: bool,
}

