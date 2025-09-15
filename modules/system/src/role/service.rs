use common::{AppError, AppResult};
use sqlx::{PgPool, Postgres, Transaction};
use tracing::info;

use crate::role::model::{RoleDTO, SysRole};
/// 新增角色，并处理其与菜单的关联关系（事务性）
pub async fn add_role(db: &PgPool, vo: RoleDTO) -> Result<u8, AppError> {
    info!("[SERVICE] Entering add_role with vo: {:?}", vo);
    // 开启数据库事务
    let mut tx = db.begin().await.map_err(AppError::DatabaseError)?;
    info!("[TX] Transaction started for adding a new role.");

    // 1. 插入角色基本信息
    let result = sqlx::query!(
        "INSERT INTO sys_role (role_name, role_key, role_sort, status, remark, create_by, create_time) VALUES ($1, $2, $3, $4, $5, 'admin', NOW()) RETURNING role_id",
        vo.role_name, vo.role_key, vo.role_sort, vo.status, vo.remark
    )
        .fetch_one(&mut *tx) // 在事务上执行
        .await?;

    let role_id = result.role_id;
    info!("[TX] Inserted into sys_role, new role_id: {}", role_id);

    // 2. 插入角色和菜单的关联信息
    if let Some(menu_ids) = vo.menu_ids.as_ref().filter(|ids| !ids.is_empty()) {
        insert_role_menu(&mut tx, role_id, menu_ids).await?;
    }
    // 提交事务
    tx.commit().await.map_err(AppError::DatabaseError)?;
    info!(
        "[TX] Transaction committed successfully for role_id: {}",
        role_id
    );
    Ok(1)
}

///根据用户id查询角色列表
pub async fn select_role_list_by_user_id(db: &PgPool, user_id: i32) -> AppResult<Vec<SysRole>> {
    info!("[SERVICE] Select role list by user id:{}", user_id);
    sqlx::query_as!(
        SysRole,
        "select sr.* from  sys_role  sr left join  sys_user_role sur  on sr.role_id = sur.role_id
              where  sur.user_id = $1",
        user_id
    )
    .fetch_all(db)
    .await
    .map_err(AppError::from)
}

/// 辅助函数：在事务中插入角色与菜单的关联记录
async fn insert_role_menu(
    tx: &mut Transaction<'_, Postgres>,
    role_id: i32,
    menu_ids: &[i32],
) -> Result<(), AppError> {
    info!(
        "[TX_HELPER] Inserting {} menu associations for role_id: {}",
        menu_ids.len(),
        role_id
    );
    // 构建批量插入的SQL
    let mut sql = "INSERT INTO sys_role_menu (role_id, menu_id) VALUES ".to_string();
    let mut values = Vec::new();
    for menu_id in menu_ids {
        values.push(format!("({}, {})", role_id, menu_id));
    }
    sql.push_str(&values.join(", "));

    sqlx::query(&sql).execute(&mut **tx).await?;
    info!("[TX_HELPER] Successfully inserted menu associations.");
    Ok(())
}

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
