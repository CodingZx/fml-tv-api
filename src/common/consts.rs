use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;

/// 统一前缀
pub const PREFIX_KEY: &str = "tv";

/// 默认ID
pub const DEFAULT_ID: &str = "00000000-0000-0000-0000-000000000000";

/// 获得默认ID
pub fn get_default_id() -> Uuid {
    Uuid::nil()
}

/// 获得默认时间
pub fn get_default_time() -> NaiveDateTime {
    let default_date = NaiveDate::from_ymd_opt(2000,1,1).unwrap();
    let default_time = NaiveTime::from_hms_opt(0,0,0).unwrap();
    NaiveDateTime::new(default_date, default_time)
}

/// Token Header
pub const TOKEN_HEADER: &str = "F-Access-Token";

pub mod conf_keys {

    pub fn easy_bangumi_conf_key() -> String {
        "config:easy_bangumi".to_string()
    }

}

pub mod env_keys {
    /// 服务端口
    pub const SERVER_PORT: &str = "SERVER_PORT";
    /// JSON大小限制 MB
    pub const SERVER_JSON_LIMIT_MB: &str = "SERVER_JSON_LIMIT_MB";

    /// 数据库 Host
    pub const DB_HOST: &str = "DB_HOST";
    /// 数据库 Port
    pub const DB_PORT: &str = "DB_PORT";
    /// 数据库 数据库名称
    pub const DB_NAME: &str = "DB_NAME";
    /// 数据库 用户名
    pub const DB_USERNAME: &str = "DB_USERNAME";
    /// 数据库 密码
    pub const DB_PASSWORD: &str = "DB_PASSWORD";
    /// 数据库 Schema
    pub const DB_SCHEMA: &str = "DB_SCHEMA";

    /// 数据库连接池最小连接数
    pub const DB_POOL_MIN_CONN: &str = "DB_POOL_MIN_CONN";
    /// 数据库连接池最大连接数
    pub const DB_POOL_MAX_CONN: &str = "DB_POOL_MAX_CONN";
    /// 数据库连接池 连接超时时间(秒)
    pub const DB_POOL_CONN_TIMEOUT_SEC: &str = "DB_POOL_CONN_TIMEOUT_SEC";
    /// 数据库连接池 获取连接超时时间(秒)
    pub const DB_POOL_ACQ_TIMEOUT_SEC: &str = "DB_POOL_ACQ_TIMEOUT_SEC";
    /// 数据库连接池 闲置连接时间(秒)
    pub const DB_POOL_IDLE_TIMEOUT_SEC: &str = "DB_POOL_IDLE_TIMEOUT_SEC";
    /// 数据库连接池 最大存活时间(秒)
    pub const DB_POOL_MAX_LIFE_TIME_SEC: &str = "DB_POOL_MAX_LIFE_TIMEOUT_SEC";
    /// 数据库 是否开启SQLX日志
    pub const DB_SQLX_LOGGING: &str = "DB_SQLX_LOGGING";
    /// 数据库 SQLX日志级别
    pub const DB_SQLX_LOGGING_LEVEL: &str = "DB_SQLX_LOGGING_LEVEL";


    /// PGQ 是否启动
    pub const PGQ_ENABLE: &str = "PGQ_ENABLE";
    /// PGQ 队列拉取间隔
    pub const PGQ_FETCH_INTERVAL_MS: &str = "PGQ_FETCH_INTERVAL_MS";
    /// PGQ 延迟队列拉取间隔
    pub const PGQ_DELAY_FETCH_INTERVAL_MS: &str = "PGQ_DELAY_FETCH_INTERVAL_MS";
    /// PGQ 自动清理
    pub const PGQ_AUTO_CLEAN: &str = "PGQ_AUTO_CLEAN";


}

pub mod queues {
    /// 操作日志队列
    pub const OPER_LOG_QUEUE: &str = "operation_log_queue";
    /// 采集队列
    pub const COLLECT_QUEUE: &str = "collect_queue";
    /// 采集视频处理队列
    pub const COLLECT_VOD_QUEUE: &str = "collect_vod_queue";
    /// 采集视频重新处理处理队列
    pub const TV_VOD_REBUILD_QUEUE: &str = "tv_vod_rebuild_queue";
}

pub mod lock_keys {
    use crate::common::consts::PREFIX_KEY;

    pub fn collect_task_key() -> String {
        format!("{}:collect:task:clean", PREFIX_KEY)
    }

    pub fn cache_clean_key() -> String {
        format!("{}:cache:expire:clean", PREFIX_KEY)
    }

    pub fn lock_clean_key() -> String {
        format!("{}:lock:expire:clean", PREFIX_KEY)
    }

}

pub mod cache_keys {
    pub mod login {
        use crate::common::consts::PREFIX_KEY;

        /// 登录IP校验
        pub fn ip_limit(ip: &str, source: &str) -> String {
            format!("{}:login:ip:limit:{}:{}", PREFIX_KEY, ip, source)
        }
    }

}

pub mod system {
    /// 账号最小长度
    pub const ADMIN_ACCOUNT_MIN_LEN: usize = 1;
    /// 账号最大长度
    pub const ADMIN_ACCOUNT_MAX_LEN: usize = 20;

    /// 密码最小长度
    pub const ADMIN_PWD_MIN_LEN: usize = 3;
    /// 密码最大长度
    pub const ADMIN_PWD_MAX_LEN: usize = 64;
}