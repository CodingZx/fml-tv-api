use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

#[allow(unused_imports)]
pub use tracing::log::Level;
#[allow(unused_imports)]
pub use tracing::log::LevelFilter;
#[allow(unused_imports)]
pub use tracing::{debug, error, info, warn};


/// 初始化日志
pub fn init() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("DEBUG"));
    // 日志配置
    let layer = fmt::layer()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .with_level(true);

    registry().with(env_filter).with(layer).init();
}

