use common::AppError;
use sqlx::PgPool;
use tracing::info;

// /// 新增角色，并处理其与菜单的关联关系（事务性）
// pub async fn add_role(db: &PgPool, vo: AddRoleVo) -> Result<u64, AppError> {
//     info!("[SERVICE] Entering add_role with vo: {:?}", vo);
//     // 开启数据库事务
//     let mut tx = db.begin().await.map_err(AppError::DatabaseError)?;
//     info!("[TX] Transaction started for adding a new role.");

//     // 1. 插入角色基本信息
//     let result = sqlx::query!(
//         "INSERT INTO sys_role (role_name, role_key, role_sort, status, remark, create_by, create_time) VALUES ($1, $2, $3, $4, $5, 'admin', NOW())",
//         vo.role_name, vo.role_key, vo.role_sort, vo.status, vo.remark
//     )
//         .execute(&mut *tx) // 在事务上执行
//         .await?;

//     let role_id = result.last_insert_id() as i64;
//     info!("[TX] Inserted into sys_role, new role_id: {}", role_id);

//     // 2. 插入角色和菜单的关联信息
//     if let Some(menu_ids) = vo.menu_ids {
//         if !menu_ids.is_empty() {
//             insert_role_menu(&mut tx, role_id, &menu_ids).await?;
//         }
//     }

//     // 提交事务
//     tx.commit().await.map_err(AppError::DatabaseError)?;
//     info!("[TX] Transaction committed successfully for role_id: {}", role_id);

//     Ok(result.rows_affected())
// }

// #[cfg(test)]
// mod user_test {
//     use framework::db::DBPool;

//     use crate::user::model::SysUserDTO;
//     #[tokio::test]
//     async fn test() -> anyhow::Result<()> {
//         let url = "postgres://postgres:Sky%402024@172.16.100.200:25432/project?options=-c%20search_path%3Dsky_website";
//         let db = DBPool::inint(url).await?;
//         SysUserDTO{ user_name: "admin".to_string(),
//             nick_name: "admin".to_string(), user_type:None
//             , email: None, phone_number: None, avatar: None, password: "admin".to_string(), status: None, remark: None, role_ids: todo!() }

//         // pub async fn add_user(db: &PgPool, sys_user_dto: SysUserDTO) -> AppResult<u64, AppError>
//         Ok(())
//     }
// }
