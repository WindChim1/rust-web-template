use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

/// 菜单权限表对应的结构体
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysMenu {
    /// 菜单ID
    pub menu_id: i32, // 对应serial类型（本质为i32）

    /// 菜单名称
    pub menu_name: String, // varchar(50)，非空

    /// 父菜单ID
    #[serde(default = "default_parent_id")]
    pub parent_id: i64, // bigint，默认0

    /// 显示顺序
    #[serde(default = "default_order_num")]
    pub order_num: i32, // integer，默认0

    /// 路由地址
    #[serde(default = "default_string")]
    pub path: String, // varchar(200)，默认空字符串

    /// 路由参数
    pub query: Option<String>, // varchar(255)，可选字段

    /// 路由名称
    #[serde(default = "default_string")]
    pub route_name: String, // varchar(50)，默认空字符串

    /// 菜单类型（M目录 C菜单 F按钮）
    #[serde(default = "default_menu_type")]
    pub menu_type: String, // char，默认空字符

    /// 菜单状态（0正常 1停用）
    #[serde(default = "default_status")]
    pub status: String, // char，默认'0'

    /// 菜单图标
    #[serde(default = "default_icon")]
    pub icon: String, // varchar(100)，默认'#'

    /// 创建者
    #[serde(default = "default_string")]
    pub create_by: String, // varchar(64)，默认空字符串

    /// 创建时间
    #[serde(default = "default_create_time")]
    pub create_time: OffsetDateTime, // timestamp with time zone，默认当前时间

    /// 更新者
    #[serde(default = "default_string")]
    pub update_by: String, // varchar(64)，默认空字符串

    /// 更新时间
    pub update_time: Option<OffsetDateTime>, // timestamp with time zone，可选字段

    /// 备注
    #[serde(default = "default_string")]
    pub remark: String, // varchar(500)，默认空字符串
}

// 默认值函数，用于serde序列化/反序列化时提供默认值
fn default_parent_id() -> i64 {
    0
}

fn default_order_num() -> i32 {
    0
}

fn default_string() -> String {
    String::new()
}

fn default_menu_type() -> String {
    String::new()
}

fn default_status() -> String {
    "0".to_string()
}

fn default_icon() -> String {
    "#".to_string()
}

fn default_create_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuTreeVo {
    #[serde(flatten)]
    pub menu: SysMenu,
    pub children: Vec<MenuTreeVo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuDTO {
    pub menu_id: i64,
    pub parent_id: Option<i64>,
    pub menu_name: String,
    pub order_num: Option<i32>,
    pub path: Option<String>,
    pub menu_type: String,
    pub status: String,
    pub icon: Option<String>,
    pub remark: Option<String>,
}
