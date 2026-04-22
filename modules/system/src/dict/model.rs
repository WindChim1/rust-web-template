use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

/// 字典类型表实体（对应 sys_dict_type 表）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")] // JSON 序列化时转小驼峰（dict_id → dictId）
pub struct SysDictType {
    /// 字典主键（PostgreSQL serial → i32，自增整数）
    pub dict_id: i32,

    /// 字典名称（VARCHAR(100)，默认空字符串）
    #[serde(default = "String::new")]
    pub dict_name: String,

    /// 字典类型（VARCHAR(100)，默认空字符串，唯一约束）
    #[serde(default = "String::new")]
    pub dict_type: String,

    /// 状态（CHAR(1)，默认 '0'，0正常 1停用）
    #[serde(default = "default_status")]
    pub status: String,

    /// 创建者（VARCHAR(64)，默认空字符串）
    #[serde(default = "String::new")]
    pub create_by: String,

    /// 创建时间（TIMESTAMP，允许 NULL）
    pub create_time: Option<OffsetDateTime>,

    /// 更新者（VARCHAR(64)，默认空字符串）
    #[serde(default = "String::new")]
    pub update_by: String,

    /// 更新时间（TIMESTAMP，允许 NULL）
    pub update_time: Option<OffsetDateTime>,

    /// 备注（VARCHAR(500)，默认 NULL）
    pub remark: Option<String>,
}

/// 创建字典类型的请求实体（不含自增主键和自动填充的时间字段）
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddSysDictTypeDTO {
    /// 字典名称（必传）
    pub dict_name: String,

    /// 字典类型（必传，唯一）
    pub dict_type: String,

    /// 状态（可选，默认 '0'）
    #[serde(default = "default_status")]
    pub status: String,

    /// 创建者（可选，默认空字符串）
    #[serde(default = "String::new")]
    pub create_by: String,

    /// 备注（可选）
    pub remark: Option<String>,
}

/// 字典类型查询条件实体（用于列表查询过滤）
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SysDictTypePageQuery {
    /// 字典名称（模糊查询，可选）
    pub dict_name: Option<String>,

    /// 字典类型（精确查询，可选）
    pub dict_type: Option<String>,

    /// 状态（精确查询，可选）
    pub status: Option<String>,
}

/// 字典数据表实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SysDictData {
    /// 字典编码（自增 BIGSERIAL → i64）
    pub dict_code: i32,

    /// 字典排序（默认 0）
    #[serde(default = "default_dict_sort")]
    pub dict_sort: i32,

    /// 字典标签（默认空字符串）
    #[serde(default = "String::new")]
    pub dict_label: String,

    /// 字典键值（默认空字符串）
    #[serde(default = "String::new")]
    pub dict_value: String,

    /// 字典类型（关联 sys_dict_type.dict_type）
    #[serde(default = "String::new")]
    pub dict_type: String,

    /// 是否默认（Y是 N否，默认 'N'）
    #[serde(default = "default_is_default")]
    pub is_default: String,

    /// 状态（0正常 1停用，默认 '0'）
    #[serde(default = "default_status")]
    pub status: String,

    /// 创建者（默认空字符串）
    #[serde(default = "String::new")]
    pub create_by: String,

    /// 创建时间（TIMESTAMP → DateTime<Utc>）
    pub create_time: Option<OffsetDateTime>,

    /// 更新者（默认空字符串）
    #[serde(default = "String::new")]
    pub update_by: String,

    /// 更新时间（TIMESTAMP → DateTime<Utc>）
    pub update_time: Option<OffsetDateTime>,

    /// 备注（默认 NULL）
    pub remark: Option<String>,
}

// 字典排序默认值（对应表中 DEFAULT 0）
fn default_dict_sort() -> i32 {
    0
}

// 是否默认字段默认值（对应表中 DEFAULT 'N'）
fn default_is_default() -> String {
    String::from("N")
}

// 状态字段默认值（复用之前的函数，或单独定义均可）
fn default_status() -> String {
    '0'.to_string()
}

/// 用于创建字典数据的请求实体
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddSysDictDataDTO {
    #[serde(default = "default_dict_sort")]
    pub dict_sort: i32,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_type: String,
    #[serde(default = "default_is_default")]
    pub is_default: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "String::new")]
    pub create_by: String,
    pub remark: Option<String>,
}
