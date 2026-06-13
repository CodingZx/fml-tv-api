use crate::common::pgq::consumer::ConsumerRegister;
use crate::common::state::AppState;
use crate::consumer::collect_consumer::CollectConsumer;
use crate::consumer::collect_vod_consumer::CollectVodConsumer;
use crate::consumer::oper_log_consumer::OperLogConsumer;
use crate::consumer::tv_vod_rebuild_consumer::TvVodRebuildConsumer;
use std::sync::Arc;

pub mod oper_log_consumer;
pub mod collect_consumer;
pub mod collect_vod_consumer;
pub mod tv_vod_rebuild_consumer;

/// 注册消费者
pub fn register_consumer(register: &mut ConsumerRegister, state: Arc<AppState>) {
    // 操作日志
    register.register(OperLogConsumer::new(Arc::clone(&state)));
    // 采集
    register.register(CollectConsumer::new(Arc::clone(&state)));
    // 处理采集结果
    register.register(CollectVodConsumer::new(Arc::clone(&state)));
    // 视频修改
    register.register(TvVodRebuildConsumer::new(Arc::clone(&state)));
}