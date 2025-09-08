use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 字典数据实体，与 `sys_dict_data` 表完全对应
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SysDictData {
    pub dict_type: Option<String>,
    pub dict_code: i64,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub is_default: Option<String>,
    pub status: Option<String>,
    pub dict_sort: Option<i32>,
    pub create_by: Option<String>,
    pub create_time: Option<OffsetDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<OffsetDateTime>,
    pub remark: Option<String>,
}
