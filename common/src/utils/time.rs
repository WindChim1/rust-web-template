use serde::{Deserialize, Deserializer, Serializer};
use time::{
    OffsetDateTime, PrimitiveDateTime, UtcOffset, format_description::well_known::Rfc3339,
    macros::format_description,
};

/// 格式常量定义（编译时检查格式合法性）
const ISO8601_STR: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

/// 通用工具函数
pub struct TimeUtil;

impl TimeUtil {
    /// 获取当前 UTC 时间
    pub fn now_utc() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }

    /// 获取当前本地时间 (默认使用 +8 偏移，可根据需求修改)
    pub fn now_local() -> OffsetDateTime {
        OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(8, 0, 0).unwrap())
    }

    /// 手动解析：尝试从字符串解析为毫秒时间戳，再转为 OffsetDateTime
    pub fn try_parse_ts_ms(s: &str) -> Option<OffsetDateTime> {
        s.parse::<i64>().ok().map(Self::from_ts_ms)
    }

    /// 手动解析：尝试从 RFC3339 字符串解析
    pub fn try_parse_rfc3339(s: &str) -> Option<OffsetDateTime> {
        OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339).ok()
    }

    /// OffsetDateTime 转 毫秒时间戳
    pub fn to_ts_ms(dt: OffsetDateTime) -> i64 {
        (dt.unix_timestamp_nanos() / 1_000_000) as i64
    }

    /// 毫秒时间戳 转 OffsetDateTime
    pub fn from_ts_ms(ms: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp_nanos((ms as i128) * 1_000_000)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH)
    }
}

// ==========================================================
// Serde 序列化适配器模块
// ==========================================================

/// 1. 处理 OffsetDateTime <-> RFC3339 字符串
pub mod offset {
    use super::*;
    pub fn serialize<S>(dt: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&dt.format(&Rfc3339).map_err(serde::ser::Error::custom)?)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)
    }
}

/// 2. 处理 Option<OffsetDateTime> <-> RFC3339 字符串 (支持空字符串 "")
pub mod opt_offset {
    use super::*;
    pub fn serialize<S>(dt: &Option<OffsetDateTime>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(d) => s.serialize_str(&d.format(&Rfc3339).map_err(serde::ser::Error::custom)?),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(d)?;
        match s.as_deref() {
            None | Some("") => Ok(None),
            Some(val) => OffsetDateTime::parse(val, &Rfc3339)
                .map(Some)
                .map_err(serde::de::Error::custom),
        }
    }
}

/// 3. 处理 PrimitiveDateTime <-> ISO8601 (无时区)
pub mod primitive {
    use super::*;
    pub fn serialize<S>(dt: &PrimitiveDateTime, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&dt.format(&ISO8601_STR).map_err(serde::ser::Error::custom)?)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        PrimitiveDateTime::parse(&s, &ISO8601_STR).map_err(serde::de::Error::custom)
    }
}

/// 4. 处理 Option<PrimitiveDateTime> <-> ISO8601 (支持空字符串 "")
pub mod opt_primitive {
    use super::*;
    pub fn serialize<S>(dt: &Option<PrimitiveDateTime>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(d) => s.serialize_str(&d.format(&ISO8601_STR).map_err(serde::ser::Error::custom)?),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Option<PrimitiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(d)?;
        match s.as_deref() {
            None | Some("") => Ok(None),
            Some(val) => PrimitiveDateTime::parse(val, &ISO8601_STR)
                .map(Some)
                .map_err(serde::de::Error::custom),
        }
    }
}

// 定义一个自定义的格式化模块（针对 Option<Date>）
time::serde::format_description!(pub date, Date, "[year]-[month]-[day]");

/// 5. 处理 OffsetDateTime <-> 毫秒时间戳数字 (i64)
pub mod ts_ms {
    use super::*;
    pub fn serialize<S>(dt: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(TimeUtil::to_ts_ms(*dt))
    }
    pub fn deserialize<'de, D>(d: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = i64::deserialize(d)?;
        Ok(TimeUtil::from_ts_ms(ms))
    }
}

/// 6. 处理 Option<OffsetDateTime> <-> 毫秒时间戳数字 (i64)
pub mod opt_ts_ms {
    use super::*;
    pub fn serialize<S>(dt: &Option<OffsetDateTime>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(d) => s.serialize_i64(TimeUtil::to_ts_ms(*d)),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms: Option<i64> = Option::deserialize(d)?;
        Ok(ms.map(TimeUtil::from_ts_ms))
    }
}
