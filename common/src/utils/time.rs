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

    // // 尝试解析完整的ISO 8601格式
    // if let Ok(dt) = OffsetDateTime::parse(s, &Iso8601::DEFAULT) {
    //     return Ok(dt);
    // }

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
