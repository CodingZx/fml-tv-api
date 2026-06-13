use actix_web::web::ServiceConfig;

mod login;
mod collect_site;
mod collect_type;
mod collect_vod;
mod sys_account;
mod sys_login_log;
mod sys_oper_log;
mod sys_queue_msg;
mod tv_group;
mod tv_type;
mod tv_vod;
mod sys_config;

/// 注册 Router
pub fn register(cfg: &mut ServiceConfig) {
    login::register(cfg);
    sys_account::register(cfg);
    sys_config::register(cfg);
    sys_oper_log::register(cfg);
    sys_queue_msg::register(cfg);
    sys_login_log::register(cfg);
    collect_site::register(cfg);
    collect_type::register(cfg);
    collect_vod::register(cfg);
    tv_group::register(cfg);
    tv_type::register(cfg);
    tv_vod::register(cfg);
}
