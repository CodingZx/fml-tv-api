use chrono::NaiveDateTime;

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct TimeUtil;

impl TimeUtil {
    /// 格式化为默认格式
    pub fn format_default(time: NaiveDateTime) -> String {
        time.format(DATETIME_FORMAT).to_string()
    }

    /// 格式化为指定格式
    pub fn format(time: NaiveDateTime, format: &str) -> String {
        time.format(format).to_string()
    }

    /// 尝试转换时间
    pub fn parse_default(time: &str) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(time, DATETIME_FORMAT).ok()
    }

    /// 尝试转换时间
    pub fn parse_format(time: &str, format: &str) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(time, format).ok()
    }
}
