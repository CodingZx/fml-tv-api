use crate::common::logger::LevelFilter;
use crate::common::{consts, logger};
use actix_web::error::{ErrorBadRequest, ErrorPayloadTooLarge, ErrorUnsupportedMediaType, JsonPayloadError};
use actix_web::web;
use sea_orm::{ConnectOptions, DbConn, DbErr};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// 读取Yaml配置
pub fn read_conf(yml_file: String) -> ServerConf {
    let mut conf = config::Config::builder()
        .add_source(config::File::new(&yml_file, config::FileFormat::Yaml))
        .build()
        .unwrap_or(Default::default())
        .try_deserialize()
        .unwrap_or(ServerConf::default());

    // Server 端口
    if let Ok(server_port) = env::var(consts::env_keys::SERVER_PORT) {
        if let Ok(server_port) = server_port.parse::<u16>() {
            conf.server.port = server_port;
        }
    }
    // JSON限制
    if let Ok(json_limit) = env::var(consts::env_keys::SERVER_JSON_LIMIT_MB) {
        if let Ok(json_limit) = json_limit.parse::<usize>() {
            conf.server.json_body_mb = json_limit;
        }
    }

    // 数据库 host
    if let Ok(db_host) = env::var(consts::env_keys::DB_HOST) {
        conf.database.host = db_host;
    }
    // 数据库 端口号
    if let Ok(db_port) = env::var(consts::env_keys::DB_PORT) {
        if let Ok(db_port) = db_port.parse::<u32>() {
            conf.database.port = db_port;
        }
    }
    // 数据库 名称
    if let Ok(db_name) = env::var(consts::env_keys::DB_NAME) {
        conf.database.dbname = db_name;
    }
    // 数据库 用户名
    if let Ok(db_username) = env::var(consts::env_keys::DB_USERNAME) {
        conf.database.username = db_username;
    }
    // 数据库 密码
    if let Ok(db_password) = env::var(consts::env_keys::DB_PASSWORD) {
        conf.database.password = db_password;
    }
    // 数据库 SCHEMA
    if let Ok(schema) = env::var(consts::env_keys::DB_SCHEMA) {
        conf.database.schema = Some(schema);
    }

    let mut pool = if let Some(pool) = conf.database.pool.clone() {
        pool
    } else {
        DBPoolConf::default()
    };
    // 连接池
    if let Ok(min_conn) = env::var(consts::env_keys::DB_POOL_MIN_CONN) {
        if let Ok(min_conn) = min_conn.parse::<u32>() {
            pool.min_conn = Some(min_conn);
        }
    }
    if let Ok(max_conn) = env::var(consts::env_keys::DB_POOL_MAX_CONN) {
        if let Ok(max_conn) = max_conn.parse::<u32>() {
            pool.max_conn = Some(max_conn);
        }
    }
    if let Ok(conn_timeout_sec) = env::var(consts::env_keys::DB_POOL_CONN_TIMEOUT_SEC) {
        if let Ok(conn_timeout_sec) = conn_timeout_sec.parse::<u64>() {
            pool.conn_timeout_sec = Some(conn_timeout_sec);
        }
    }
    if let Ok(acq_timeout_sec) = env::var(consts::env_keys::DB_POOL_ACQ_TIMEOUT_SEC) {
        if let Ok(acq_timeout_sec) = acq_timeout_sec.parse::<u64>() {
            pool.acq_timeout_sec = Some(acq_timeout_sec);
        }
    }
    if let Ok(idle_timeout_sec) = env::var(consts::env_keys::DB_POOL_IDLE_TIMEOUT_SEC) {
        if let Ok(idle_timeout_sec) = idle_timeout_sec.parse::<u64>() {
            pool.idle_timeout_sec = Some(idle_timeout_sec);
        }
    }
    if let Ok(max_life_time_sec) = env::var(consts::env_keys::DB_POOL_MAX_LIFE_TIME_SEC) {
        if let Ok(max_life_time_sec) = max_life_time_sec.parse::<u64>() {
            pool.max_lifetime_sec = Some(max_life_time_sec);
        }
    }
    if let Ok(sqlx_logging) = env::var(consts::env_keys::DB_SQLX_LOGGING) {
        if let Ok(sqlx_logging) = sqlx_logging.parse::<bool>() {
            pool.sqlx_logging = Some(sqlx_logging);
        }
    }
    if let Ok(sqlx_logging_level) = env::var(consts::env_keys::DB_SQLX_LOGGING_LEVEL) {
        pool.sqlx_logging_level = Some(sqlx_logging_level);
    }
    conf.database.pool = Some(pool);


    // PGQ配置
    if let Ok(pgq_enable) = env::var(consts::env_keys::PGQ_ENABLE) {
        if let Ok(pgq_enable) = pgq_enable.parse::<bool>() {
            conf.pgq.enable = pgq_enable;
        }
    }
    if let Ok(pgq_fetch_interval_ms) = env::var(consts::env_keys::PGQ_FETCH_INTERVAL_MS) {
        if let Ok(pgq_fetch_interval_ms) = pgq_fetch_interval_ms.parse::<u64>() {
            conf.pgq.fetch_interval_ms = pgq_fetch_interval_ms;
        }
    }
    if let Ok(pgq_delay_fetch_interval_ms) = env::var(consts::env_keys::PGQ_DELAY_FETCH_INTERVAL_MS) {
        if let Ok(pgq_delay_fetch_interval_ms) = pgq_delay_fetch_interval_ms.parse::<u64>() {
            conf.pgq.delay_fetch_interval_ms = pgq_delay_fetch_interval_ms;
        }
    }
    if let Ok(pgq_auto_clean) = env::var(consts::env_keys::PGQ_AUTO_CLEAN) {
        if let Ok(pgq_auto_clean) = pgq_auto_clean.parse::<bool>() {
            conf.pgq.auto_clean = pgq_auto_clean;
        }
    }

    conf
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConf {
    /// Actix-Web相关配置
    pub server: ActixConf,
    /// 数据库相关配置
    pub database: DatabaseConf,
    /// PGQ 相关配置
    pub pgq: PgqConf,
}

impl Default for ServerConf {
    fn default() -> Self {
        Self {
            server: ActixConf::default(),
            database: DatabaseConf::default(),
            pgq: PgqConf::default(),
        }
    }
}

/// Actix-Web相关配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActixConf {
    pub port: u16,
    #[serde(rename(deserialize = "json-body-mb"))]
    pub json_body_mb: usize,
}

impl Default for ActixConf {
    fn default() -> Self {
        Self {
            port: 18099,
            json_body_mb: 5,
        }
    }
}

impl ActixConf {

    pub fn json_config(&self) -> web::JsonConfig {
        web::JsonConfig::default()
            .limit(self.json_body_mb * 1024 * 1024)
            .content_type(|m| m == "application/json")
            .error_handler(|err, req| {
                logger::warn!("JSONError: {} {}", req.path(), err);
                match err {
                    JsonPayloadError::OverflowKnownLength { .. } => ErrorPayloadTooLarge("JSON payload is too larger"),
                    JsonPayloadError::Overflow { .. } => ErrorPayloadTooLarge("JSON payload is too larger"),
                    JsonPayloadError::ContentType => ErrorUnsupportedMediaType("Content type error"),
                    _ => ErrorBadRequest(err),
                }
            })
    }

}


/// 数据库相关配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConf {
    pub host: String,
    pub port: u32,
    pub dbname: String,
    pub username: String,
    pub password: String,
    pub schema: Option<String>,
    pub pool: Option<DBPoolConf>,
}

impl Default for DatabaseConf {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            dbname: "fml-tv".to_string(),
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            schema: Some("public".to_string()),
            pool: Some(DBPoolConf::default()),
        }
    }
}


/// 数据库连接池相关配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBPoolConf {
    #[serde(rename(deserialize = "min-conn"))]
    pub min_conn: Option<u32>,
    #[serde(rename(deserialize = "max-conn"))]
    pub max_conn: Option<u32>,
    #[serde(rename(deserialize = "conn-timeout-sec"))]
    pub conn_timeout_sec: Option<u64>,
    #[serde(rename(deserialize = "acq-timeout-sec"))]
    pub acq_timeout_sec: Option<u64>,
    #[serde(rename(deserialize = "idle-timeout-sec"))]
    pub idle_timeout_sec: Option<u64>,
    #[serde(rename(deserialize = "max-lifetime-sec"))]
    pub max_lifetime_sec: Option<u64>,
    #[serde(rename(deserialize = "sqlx-logging"))]
    pub sqlx_logging: Option<bool>,
    #[serde(rename(deserialize = "sqlx-logging-level"))]
    pub sqlx_logging_level: Option<String>,
}

impl Default for DBPoolConf {
    fn default() -> Self {
        Self {
            min_conn: Some(1),
            max_conn: Some(10),
            conn_timeout_sec: Some(10),
            acq_timeout_sec: Some(10),
            idle_timeout_sec: Some(30),
            max_lifetime_sec: Some(3600),
            sqlx_logging: Some(false),
            sqlx_logging_level: Some("off".to_string()),
        }
    }
}

impl DatabaseConf {

    fn to_conn_url(&self) -> String {
        // postgres://username:password@host/database
        let db_username = &self.username;
        let db_password = &self.password;
        let db_host = &self.host;
        let db_port = &self.port;
        let db_name = &self.dbname;
        format!("postgres://{db_username}:{db_password}@{db_host}:{db_port}/{db_name}")
    }

    pub async fn connect(&self) -> Result<DbConn, DbErr> {
        let mut opt = ConnectOptions::new(self.to_conn_url());
        if let Some(schema) = &self.schema {
            opt.set_schema_search_path(schema);
        }

        if let Some(pool_conf) = &self.pool {
            if let Some(max_conn) = pool_conf.max_conn {
                opt.max_connections(max_conn);
            }
            if let Some(min_conn) = pool_conf.min_conn {
                opt.min_connections(min_conn);
            }
            if let Some(conn_timeout_sec) = pool_conf.conn_timeout_sec {
                opt.connect_timeout(Duration::from_secs(conn_timeout_sec));
            }
            if let Some(acq_timeout_sec) = pool_conf.acq_timeout_sec {
                opt.acquire_timeout(Duration::from_secs(acq_timeout_sec));
            }
            if let Some(idle_timeout_sec) = pool_conf.idle_timeout_sec {
                opt.idle_timeout(Duration::from_secs(idle_timeout_sec));
            }
            if let Some(max_lifetime_sec) = pool_conf.max_lifetime_sec {
                opt.max_lifetime(Duration::from_secs(max_lifetime_sec));
            }
            if let Some(sqlx_logging) = pool_conf.sqlx_logging {
                opt.sqlx_logging(sqlx_logging);
            } else {
                opt.sqlx_logging(false);
            }
            if let Some(sqlx_log_level) = &pool_conf.sqlx_logging_level {
                opt.sqlx_logging_level(Self::level_filter_from_str(&sqlx_log_level.to_ascii_lowercase()));
            }
        }
        let pool = sea_orm::Database::connect(opt).await?;
        logger::debug!("Db pool created...");
        Ok(pool)
    }

    fn level_filter_from_str(s: &str) -> LevelFilter {
        match s {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Off,
        }
    }
}

/// PGQ相关配置
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PgqConf {
    pub enable: bool,
    #[serde(rename(deserialize = "fetch-interval-ms"))]
    pub fetch_interval_ms: u64,
    #[serde(rename(deserialize = "delay-fetch-interval-ms"))]
    pub delay_fetch_interval_ms: u64,
    #[serde(rename(deserialize = "auto-clean"))]
    pub auto_clean: bool,
}

impl Default for PgqConf {
    fn default() -> Self {
        PgqConf {
            enable: false,
            fetch_interval_ms: 5000,
            delay_fetch_interval_ms: 5000,
            auto_clean: true,
        }
    }
}