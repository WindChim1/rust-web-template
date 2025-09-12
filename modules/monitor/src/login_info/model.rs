use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 登录日志记录实体，与 `sys_logininfor` 数据库表完全对应。
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysLoginInfor {
    pub info_id: i64,
    pub user_name: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    // status 在数据库中是 char(1)，用 String 类型可以安全映射
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<OffsetDateTime>,
}

/// 用于登录日志列表查询的参数结构体
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListLogininforQuery {
    pub ipaddr: Option<String>,
    pub user_name: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "params[beginTime]")]
    pub begin_time: Option<String>,
    #[serde(rename = "params[endTime]")]
    pub end_time: Option<String>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
