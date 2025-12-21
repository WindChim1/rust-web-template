use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 登录日志记录实体，与 `sys_logininfor` 数据库表完全对应。
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, ToSchema)]
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
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListLogininforQuery {
    pub user_name: Option<String>,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
