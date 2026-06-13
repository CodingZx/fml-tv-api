use crate::common::error::ServerError;
use crate::common::result::ResultData;

pub mod conf;
pub mod consts;
pub mod cron;
pub mod deserialize;
pub mod error;
pub mod handler;
pub mod log_info;
pub mod logger;
pub mod model;
pub mod pgcache;
pub mod pglock;
pub mod pgq;
pub mod result;
pub mod state;
pub mod token;
pub mod util;

/// 业务返回包装
pub type ServerResult<T> = Result<T, ServerError>;

/// API返回包装
pub type Response<T> = Result<ResultData<T>, ServerError>;

#[allow(unused_imports)]
/// 将任意类型反序列化为bool
pub use deserialize::bool_from_any;
/// 反序列化方法
#[allow(unused_imports)]
/// 将任意类型反序列化为Option<bool>
pub use deserialize::option_bool_from_any;

#[allow(unused_imports)]
/// 将任意类型反序列化为Option<String>
pub use deserialize::option_str_from_any;
#[allow(unused_imports)]
/// 将任意类型反序列化为String
pub use deserialize::str_from_any;

#[allow(unused_imports)]
/// 将任意类型反序列化为Option<u64>
pub use deserialize::option_u64_from_any;
#[allow(unused_imports)]
/// 将任意类型反序列化为u64
pub use deserialize::u64_from_any;

#[allow(unused_imports)]
/// 将任意类型反序列化为i64
pub use deserialize::i64_from_any;
#[allow(unused_imports)]
/// 将任意类型反序列化为Option<i64>
pub use deserialize::option_i64_from_any;

#[allow(unused_imports)]
/// 将任意类型反序列化为i32
pub use deserialize::i32_from_any;
#[allow(unused_imports)]
/// 将任意类型反序列化为Option<i32>
pub use deserialize::option_i32_from_any;

#[allow(unused_imports)]
/// 将任意类型反序列化为Option<u32>
pub use deserialize::option_u32_from_any;
#[allow(unused_imports)]
/// 将任意类型反序列化为u32
pub use deserialize::u32_from_any;
