use crate::common::consts::cache_keys;
use crate::common::consts::system::{ADMIN_ACCOUNT_MAX_LEN, ADMIN_PWD_MAX_LEN};
use crate::common::error::ServerError::BusinessStrError;
use crate::common::model::LoginIpLimit;
use crate::common::pgcache::service::DbCache;
use crate::common::state::AppState;
use crate::common::ServerResult;
use crate::database::dao::sys_login_log::SysLoginLogDao;
use crate::database::model::sys_login_log::SysLoginLogActiveModel;
use actix_web::HttpRequest;
use chrono::Local;
use get_if_addrs::IfAddr;
use sea_orm::Set;
use std::time::Duration;
use uuid::Uuid;

pub struct IPUtil;

impl IPUtil {
    /// 获取IP地址
    pub fn get_ip(req: &HttpRequest) -> String {
        req.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_default()
    }

    /// 获取本机IP地址
    pub fn get_local_ip() -> String {
        let addrs = match get_if_addrs::get_if_addrs() {
            Ok(v) => v,
            Err(_) => return "127.0.0.1".to_string(),
        };

        for ifa in addrs {
            if ifa.is_loopback() {
                continue;
            }
            if let IfAddr::V4(addr) = ifa.addr {
                return addr.ip.to_string();
            }
        }

        String::from("127.0.0.1")
    }

    /// 校验IP限制
    pub async fn admin_check_ip_limit(state: &AppState, ip: &str) -> ServerResult<()> {
        Self::check_ip_limit(state, ip, ADMIN_SOURCE, 5).await
    }

    /// 校验IP限制
    pub async fn app_check_ip_limit(state: &AppState, ip: &str) -> ServerResult<()> {
        Self::check_ip_limit(state, ip, APP_SOURCE, 10).await
    }

    /// 校验IP限制
    async fn check_ip_limit(state: &AppState, ip: &str, source: &str, max_count: u32) -> ServerResult<()> {
        let key = cache_keys::login::ip_limit(ip, source);
        let ops = DbCache::new(&state.db, &key);
        if let Some(val) = ops.get_str().await? {
            let cache: LoginIpLimit = serde_json::from_str(&val)?;
            if cache.count >= max_count {
                let cache_time = cache.last_time + Duration::from_mins(30);
                let now = Local::now().naive_local();
                let seconds = (cache_time - now).num_seconds();
                if seconds < 0 {
                    ops.delete().await?;
                    return Ok(());
                }
                let error_msg = format!("请求失败次数过多, 请等待{seconds}秒后再试");
                return Err(BusinessStrError(error_msg))
            }
        }
        Ok(())
    }

    pub async fn admin_set_ip_limit(state: &AppState, ip: &str, username: String, password: String) -> ServerResult<()> {
        Self::set_ip_limit(state, ADMIN_SOURCE, ip, username, password).await
    }

    pub async fn app_set_ip_limit(state: &AppState, ip: &str) -> ServerResult<()> {
        Self::set_ip_limit(state, APP_SOURCE, ip, "".to_string(), "".to_string()).await
    }

    async fn set_ip_limit(state: &AppState, source: &str, ip: &str, username: String, password: String) -> ServerResult<()> {
        {
            let key = cache_keys::login::ip_limit(ip, source);
            let ops = DbCache::new(&state.db, &key);
            let mut cache = match ops.get_str().await? {
                None => LoginIpLimit::default(),
                Some(val) => serde_json::from_str(&val)?,
            };
            cache.count += 1;
            cache.last_time = Local::now().naive_local();

            ops.set_json(&cache, Duration::from_hours(1)).await?;
        }
        if source != ADMIN_SOURCE {
            // 管理后台保存登录失败的账号密码,  其他的只统计登录失败次数
            return Ok(());
        }
        {
            let save_user_name = if username.chars().count() > ADMIN_ACCOUNT_MAX_LEN {
                let user_name = username.chars().take(10).collect::<String>();
                format!("{}...", user_name)
            } else {
                username
            };
            let save_password = if password.chars().count() > ADMIN_PWD_MAX_LEN {
                let pwd = password.chars().take(20).collect::<String>();
                format!("{}...", pwd)
            } else {
                password
            };

            let log = SysLoginLogActiveModel {
                id: Set(Uuid::now_v7()),
                ip_addr: Set(ip.to_string()),
                user_name: Set(save_user_name),
                password: Set(save_password),
                create_time: Set(Local::now().naive_local()),
            };
            SysLoginLogDao::new(&state.db).insert(log).await?;
        }

        Ok(())
    }
}

const APP_SOURCE: &str = "expose";
const ADMIN_SOURCE: &str = "admin";