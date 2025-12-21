use time::{OffsetDateTime, macros::offset};

use crate::{AppError, AppResult};

/// 灵活的时间解析函数
pub fn flexible_parse_datetime(s: &str) -> AppResult<Option<OffsetDateTime>> {
    // 尝试各种常见格式
    let formats = [
        // ISO 8601格式
        // time::format_description::well_known::Iso8601::DEFAULT,
        // 自定义格式：YYYY-MM-DD HH:MM:SS
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?,
        time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]")?,
        time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]+08:00")?,
        // 自定义格式：YYYY/MM/DD HH:MM:SS
        time::format_description::parse("[year]/[month]/[day] [hour]:[minute]:[second]")?,
    ];

    for format in formats.iter() {
        // 尝试解析为OffsetDateTime（带时区）
        if let Ok(dt) = OffsetDateTime::parse(s, format) {
            return Ok(Some(dt));
        }

        // 尝试解析为PrimitiveDateTime（不带时区）
        if let Ok(primitive_dt) = time::PrimitiveDateTime::parse(s, format) {
            // 假设使用本地时区
            let dt = primitive_dt.assume_offset(offset!(UTC));
            return Ok(Some(dt));
        }
    }

    Err(AppError::Other(format!("无法解析时间：{}", s)))
}

pub mod timestamp_millis_i64 {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    // 序列化：把时间转成毫秒数字 (i64)
    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => {
                // 核心逻辑：先把纳秒转成毫秒，强转成 i64
                let millis = (dt.unix_timestamp_nanos() / 1_000_000) as i64;
                serializer.serialize_i64(millis)
            }
            None => serializer.serialize_none(),
        }
    }

    // 反序列化：把毫秒数字 (i64) 转成时间
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 关键点：这里我们显式告诉 serde 我们要读 i64，而不是 i128
        let millis: Option<i64> = Option::deserialize(deserializer)?;

        match millis {
            Some(val) => {
                // 转回纳秒 (i128) 用于生成时间，这里 Rust 内部计算支持 i128 没问题
                let nanos = (val as i128) * 1_000_000;
                OffsetDateTime::from_unix_timestamp_nanos(nanos)
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
            None => Ok(None),
        }
    }
}

#[test]
fn pase_time_test() -> anyhow::Result<()> {
    // 使用示例
    let test_cases = [
        "2023-01-01 00:00:00",
        "2023-01-01T00:00:00",
        "2023-01-01T00:00:00+08:00",
        "2023/01/01 00:00:00",
    ];

    for case in test_cases {
        match flexible_parse_datetime(case) {
            Ok(dt) => println!("Success: '{}' -> {:?}", case, dt),
            Err(e) => println!("Failed: '{}' -> {}", case, e),
        }
    }

    Ok(())
}
