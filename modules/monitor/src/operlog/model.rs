use common::utils::time::opt_ts_ms;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 操作日志记录实体，与 `sys_oper_log` 数据库表完全对应。
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct SysOperLog {
    pub oper_id: i32,
    //模块标题
    pub title: Option<String>,
    //业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i16>,
    //方法名称
    pub method: Option<String>,
    //请求方式
    pub request_method: Option<String>,
    //请求方式
    pub operator_type: Option<i16>,
    pub oper_name: Option<String>,
    pub oper_nick_name: Option<String>,
    //请求地址
    pub oper_url: Option<String>,
    //主机ip
    pub oper_ip: Option<String>,
    //操作位置
    pub oper_location: Option<String>,
    //操作人员
    pub oper_param: Option<String>,
    //返回参数
    pub json_result: Option<String>,
    //请求状态
    pub status: Option<i16>,
    //错误消息
    pub error_msg: Option<String>,
    //操作时间
    pub oper_time: Option<OffsetDateTime>,
    //消耗时间
    pub cost_time: Option<i64>,
}

/// 用于操作日志列表查询的参数结构体
#[derive(Deserialize, Debug, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ListOperLogQuery {
    /// 模块标题
    pub title: Option<String>,
    /// 操作者账号
    pub oper_name: Option<String>,
    /// 操作者昵称
    pub oper_nick_name: Option<String>,
    ///业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i32>,
    /// 操作状态（0正常 1异常）
    pub status: Option<i32>,
    /// 日期范围查询
    #[serde(with = "opt_ts_ms")]
    pub start_time: Option<OffsetDateTime>,
    #[serde(with = "opt_ts_ms")]
    pub end_time: Option<OffsetDateTime>,
}

/// 操作日志记录实体，与 `sys_oper_log` 数据库表完全对应。
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OperLogVO {
    ///模块标题
    pub title: Option<String>,
    ///业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i16>,
    ///方法名称
    pub method: Option<String>,
    ///请求方式
    pub request_method: Option<String>,
    ///请求方式
    pub operator_type: Option<i16>,
    ///操作人员
    pub oper_name: Option<String>,
    pub oper_nick_name: Option<String>,
    ///请求地址
    pub oper_url: Option<String>,
    ///主机ip
    pub oper_ip: Option<String>,
    ///操作人员
    pub oper_param: Option<String>,
    ///返回参数
    pub json_result: Option<String>,
    ///请求状态
    pub status: Option<i16>,
    ///错误消息
    pub error_msg: Option<String>,
    ///操作时间
    #[serde(with = "opt_ts_ms")]
    pub oper_time: Option<OffsetDateTime>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OperLogDTO {
    //模块标题
    pub title: Option<String>,
    //业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i16>,
    //方法名称
    pub method: Option<String>,
    //请求方式
    pub request_method: Option<String>,
    //操作类别（0其它 1后台用户 2手机端用户）
    pub operator_type: Option<i16>,
    //操作人员
    pub oper_name: Option<String>,
    pub oper_nick_name: Option<String>,
    //请求地址
    pub oper_url: Option<String>,
    //主机ip
    pub oper_ip: Option<String>,
    //操作位置
    pub oper_location: Option<String>,
    //操作人员
    pub oper_param: Option<String>,
    //返回参数
    pub json_result: Option<String>,
    //请求状态
    pub status: Option<i16>,
    //错误消息
    pub error_msg: Option<String>,
    //操作时间
    #[serde(with = "opt_ts_ms")]
    pub oper_time: Option<OffsetDateTime>,
    //消耗时间
    pub cost_time: Option<i64>,
}
impl From<SysOperLog> for OperLogVO {
    fn from(value: SysOperLog) -> Self {
        let SysOperLog {
            title,
            business_type,
            method,
            request_method,
            oper_name,
            oper_nick_name,
            oper_url,
            oper_ip,
            oper_param,
            json_result,
            status,
            error_msg,
            oper_time,
            operator_type,
            ..
        } = value;
        Self {
            title,
            business_type,
            operator_type,
            method,
            request_method,
            oper_name,
            oper_nick_name,
            oper_url,
            oper_ip,
            oper_param,
            json_result,
            status,
            error_msg,
            oper_time,
        }
    }
}

pub enum BusinessType {
    Add,
    Update,
    Delete,
    Other,
}
impl BusinessType {
    pub fn get_value(&self) -> i16 {
        match self {
            BusinessType::Add => 1,
            BusinessType::Update => 2,
            BusinessType::Delete => 3,
            BusinessType::Other => 0,
        }
    }
}
