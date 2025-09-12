use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 操作日志记录实体，与 `sys_oper_log` 数据库表完全对应。
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLog {
    pub oper_id: i64,
    pub title: Option<String>,
    pub business_type: Option<i32>,
    pub method: Option<String>,
    pub request_method: Option<String>,
    pub operator_type: Option<i32>,
    pub oper_name: Option<String>,
    pub oper_url: Option<String>,
    pub oper_ip: Option<String>,
    pub oper_location: Option<String>,
    pub oper_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<i32>,
    pub error_msg: Option<String>,
    pub oper_time: Option<OffsetDateTime>,
    pub cost_time: Option<i64>,
}

/// 用于操作日志列表查询的参数结构体
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListOperLogQuery {
    // 业务查询参数
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub business_type: Option<i32>,
    pub status: Option<i32>,
    // 日期范围查询
    #[serde(rename = "params[beginTime]")]
    pub begin_time: Option<String>,
    #[serde(rename = "params[endTime]")]
    pub end_time: Option<String>,
    // 分页参数
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
