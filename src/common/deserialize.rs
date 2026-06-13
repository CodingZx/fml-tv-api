use serde::Deserializer;
use std::str::FromStr;

/// 将任意类型转换成 Option bool
pub fn option_bool_from_any<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct OptionBoolVisitor;

    impl<'de> Visitor<'de> for OptionBoolVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(value))
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                0 => Ok(Some(false)),
                1 => Ok(Some(true)),
                _ => Err(E::custom(format!("invalid boolean value: {}", value))),
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                0 => Ok(Some(false)),
                1 => Ok(Some(true)),
                _ => Err(E::custom(format!("invalid boolean value: {}", value))),
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value.to_lowercase().as_str() {
                "true" | "yes" | "1" | "on" => Ok(Some(true)),
                "false" | "no" | "0" | "off" | "" => Ok(Some(false)),
                _ => Err(E::custom(format!("invalid boolean string: {}", value))),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&value)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionBoolVisitor)
}

/// 将任意类型转换成 bool
pub fn bool_from_any<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value)
        }
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom(format!("invalid boolean value: {}", value))),
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom(format!("invalid boolean value: {}", value))),
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value.to_lowercase().as_str() {
                "true" | "yes" | "1" | "on" => Ok(true),
                "false" | "no" | "0" | "off" | "" => Ok(false),
                _ => Err(E::custom(format!("invalid boolean string: {}", value))),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&value)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to bool"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to bool"))
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}

/// 将任意类型转换成 Option u64
pub fn option_u64_from_any<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct OptionU64Visitor;

    impl<'de> Visitor<'de> for OptionU64Visitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 {
                Err(Error::custom("negative number cannot be converted to u64"))
            } else {
                Ok(Some(v as u64))
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            u64::from_str(v).map(Some).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionU64Visitor)
}

/// 将任意类型转换成 u64
pub fn u64_from_any<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct U64Visitor;

    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 {
                Err(Error::custom("negative number cannot be converted to u64"))
            } else {
                Ok(v as u64)
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            u64::from_str(v).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to u64"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to u64"))
        }
    }

    deserializer.deserialize_any(U64Visitor)
}

/// 将任意类型转换成 Option i64
pub fn option_i64_from_any<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct OptionI64Visitor;

    impl<'de> Visitor<'de> for OptionI64Visitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= i64::MAX as u64 {
                Ok(Some(v as i64))
            } else {
                Err(Error::custom(format!("u64 {} out of range for i64", v)))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            i64::from_str(v).map(Some).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionI64Visitor)
}


/// 将任意类型转换成 i64
pub fn i64_from_any<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct I64Visitor;

    impl<'de> Visitor<'de> for I64Visitor {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= i64::MAX as u64 {
                Ok(v as i64)
            } else {
                Err(Error::custom(format!("u64 {} out of range for i64", v)))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            i64::from_str(v).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to i64"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to i64"))
        }
    }

    deserializer.deserialize_any(I64Visitor)
}


/// 将任意类型转换成 Option i32
pub fn option_i32_from_any<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct OptionI32Visitor;

    impl<'de> Visitor<'de> for OptionI32Visitor {
        type Value = Option<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= i32::MAX as i64 {
                Ok(Some(v as i32))
            } else {
                Err(Error::custom(format!("u64 {} out of range for i32", v)))
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= i32::MAX as u64 {
                Ok(Some(v as i32))
            } else {
                Err(Error::custom(format!("u64 {} out of range for i32", v)))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            i32::from_str(v).map(Some).map_err(Error::custom)
        }

        // 处理字符串的另一种形式（可选，提高兼容性）
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionI32Visitor)
}

/// 将任意类型转换成 i32
pub fn i32_from_any<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct I32Visitor;

    impl<'de> Visitor<'de> for I32Visitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= i32::MAX as i64 {
                Ok(v as i32)
            } else {
                Err(Error::custom(format!("u64 {} out of range for i32", v)))
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= i32::MAX as u64 {
                Ok(v as i32)
            } else {
                Err(Error::custom(format!("u64 {} out of range for i32", v)))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            i32::from_str(v).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }


        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to i32"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to i32"))
        }
    }

    deserializer.deserialize_any(I32Visitor)
}

/// 将任意类型转换成 Option<u32>
pub fn option_u32_from_any<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct OptionU32Visitor;

    impl<'de> Visitor<'de> for OptionU32Visitor {
        type Value = Option<u32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        // 处理其他整数类型（如 i64）也可以转换
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 {
                Err(Error::custom("negative number cannot be converted to u64"))
            } else if v <= u32::MAX as i64 {
                Ok(Some(v as u32))
            } else {
                Err(Error::custom(format!("u64 {} out of range for u32", v)))
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= u32::MAX as u64 {
                Ok(Some(v as u32))
            } else {
                Err(Error::custom(format!("u64 {} out of range for u32", v)))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            u32::from_str(v).map(Some).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionU32Visitor)
}

/// 将任意类型转换成 u32
pub fn u32_from_any<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct U32Visitor;

    impl<'de> Visitor<'de> for U32Visitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        // 处理其他整数类型（如 i64）也可以转换
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 {
                Err(Error::custom("negative number cannot be converted to u64"))
            } else if v <= u32::MAX as i64 {
                Ok(v as u32)
            } else {
                Err(Error::custom(format!("u64 {} out of range for u32", v)))
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= u32::MAX as u64 {
                Ok(v as u32)
            } else {
                Err(Error::custom(format!("u64 {} out of range for u32", v)))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            u32::from_str(v).map_err(Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to u32"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to u32"))
        }
    }

    deserializer.deserialize_any(U32Visitor)
}

/// 将任意类型转换成 Option String
pub fn option_str_from_any<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct OptionStrVisitor;

    impl<'de> Visitor<'de> for OptionStrVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionStrVisitor)
}



/// 将任意类型转换成 String
pub fn str_from_any<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    use std::fmt;

    struct StrVisitor;

    impl<'de> Visitor<'de> for StrVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number, a string, or null")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.to_string())
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.to_string())
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.to_string())
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to String"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::custom("null cannot be converted to String"))
        }
    }

    deserializer.deserialize_any(StrVisitor)
}
