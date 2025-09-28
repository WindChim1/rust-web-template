use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::role::model::SysRole;

/// 系统用户实体类（严格映射数据库可空性）
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUser {
    /// 用户ID
    pub user_id: i32,

    /// 用户账号
    pub user_name: String,

    /// 用户昵称
    pub nick_name: String,

    /// 用户类型（00系统用户、01普通用户、02临时用户） 默认00
    #[serde(default = "default_user_type")]
    pub user_type: Option<String>,

    /// 用户邮箱
    pub email: Option<String>,

    /// 手机号码
    pub phone_number: Option<String>,

    /// 头像地址
    pub avatar: Option<String>,

    /// 密码（非空，存储加密后的值）
    pub password: Option<String>,

    /// 账号状态 （0正常 1停用,默认 '0'）
    #[serde(default = "default_status")]
    pub status: Option<String>,
    /// '删除标志（0代表存在 2代表删除 默认 '0'）
    #[serde(default = "default_del_flag")]
    pub del_flag: Option<String>,

    /// 最后登录IP地址
    pub login_ip: Option<String>,

    /// 最后登录时间
    pub login_date: Option<OffsetDateTime>,

    /// 密码最后更新时间
    pub pwd_update_date: Option<OffsetDateTime>,

    /// 创建者
    pub create_by: Option<String>,

    /// 记录创建时间
    pub create_time: Option<OffsetDateTime>,

    /// 更新者
    pub update_by: Option<String>,

    /// 记录更新时间
    pub update_time: Option<OffsetDateTime>,

    /// 备注信息
    pub remark: Option<String>,
}

// 非空字段的默认值（与数据库默认值保持一致）
fn default_user_type() -> Option<String> {
    Some("00".to_string())
}
fn default_status() -> Option<String> {
    Some("0".to_string())
}
fn default_del_flag() -> Option<String> {
    Some("0".to_string())
}

/// 用户响应VO（隐藏敏感字段）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUserVO {
    pub user_id: i32,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub login_ip: Option<String>,
    // #[serde(with = "time::serde::rfc3339")]
    pub login_date: Option<OffsetDateTime>,
    // #[serde(with = "time::serde::rfc3339")]
    pub create_time: Option<OffsetDateTime>,
    pub remark: Option<String>,
    pub role_list: Option<Vec<SysRole>>,
}

// 从实体类转换为响应DTO
impl From<SysUser> for SysUserVO {
    fn from(user: SysUser) -> Self {
        Self {
            user_id: user.user_id,
            user_name: user.user_name,
            nick_name: user.nick_name,
            user_type: user.user_type,
            email: user.email,
            phone_number: user.phone_number,
            avatar: user.avatar,
            status: user.status,
            login_ip: user.login_ip,
            login_date: user.login_date,
            create_time: user.create_time,
            remark: user.remark,
            role_list: None,
        }
    }
}

// 请求DTO也同步调整为 Option 类型
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUserDTO {
    pub user_name: String,
    pub nick_name: String,
    #[serde(default = "default_user_type")]
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar: Option<String>,
    pub password: String, // 明文密码，需加密
    #[serde(default = "default_status")]
    pub status: Option<String>,
    pub remark: Option<String>,
    pub role_ids: Option<Vec<i32>>, // 关联的角色ID列表
}
