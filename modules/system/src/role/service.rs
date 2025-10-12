use common::{AppError, AppResult, SqlBuilder, page_reponse::PageReponse, utils::time};
use sqlx::{PgPool, Postgres, Transaction};
use tracing::info;

use crate::role::model::{ListRoleQuery, RoleDTO, SysRole};
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
/// 辅助函数：在事务中删除角色与菜单的关联记录
async fn delete_role_menu_by_role_id(
    tx: &mut Transaction<'_, Postgres>,
    role_id: i32,
) -> Result<u64, AppError> {
    let result = sqlx::query!("DELETE FROM sys_role_menu WHERE role_id = $1", role_id)
        .execute(&mut **tx)
        .await?;
    Ok(result.rows_affected())
}

///更新角色，并处理其与菜单的关联关系（事务性）
pub(crate) async fn update_role(db: &PgPool, role: RoleDTO) -> AppResult<u64> {
    info!("[SERVICE] Entering update_role with role: {:?}", role);
    let mut tx = db.begin().await?;
    let result = sqlx::query!(
            r#"
            UPDATE sys_role
            SET role_name = $1, role_key = $2, role_sort = $3, status = $4, remark = $5, update_by = 'admin', update_time = NOW()
            WHERE role_id = $6
            "#,
            role.role_name,
            role.role_key,
            role.role_sort,
            role.status,
            role.remark,
            role.role_id
        )
        .execute(&mut *tx)
        .await?;
    // 先删除旧的关联
    if let Some(menu_ids) = role
        .menu_ids
        .as_ref()
        .filter(|menu_ids| !menu_ids.is_empty())
    {
        // 先删除旧的关联
        delete_role_menu_by_role_id(&mut tx, role.role_id.unwrap()).await?;
        // 再插入新的关联
        insert_role_menu(&mut tx, role.role_id.unwrap(), menu_ids).await?;
    }
    tx.commit().await?;
    info!("[SERVICE] Role updated successfully: {:?}", role);
    Ok(result.rows_affected())
}

/// 删除角色，并处理其与菜单的关联关系（事务性）
pub(crate) async fn delete_role(db: &PgPool, role_id: i32) -> AppResult<u64, AppError> {
    info!("[SERVICE] Entering delete_role with role_id: {}", role_id);
    let mut tx = db.begin().await?;
    // 先删除角色与菜单的关联
    delete_role_menu_by_role_id(&mut tx, role_id).await?;
    // 再删除角色本身
    let result = sqlx::query!("DELETE FROM sys_role WHERE role_id = $1", role_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(result.rows_affected())
}

/// 根据角色ID查询角色详情
pub(crate) async fn select_by_id(db: &PgPool, role_id: i32) -> AppResult<SysRole> {
    info!("[SERVICE] Entering get_by_id with role_id: {}", role_id);
    let role = sqlx::query_as!(
        SysRole,
        "SELECT * FROM sys_role WHERE role_id = $1",
        role_id
    )
    .fetch_one(db)
    .await?;
    Ok(role)
}
///根据角色id查询菜单列表
pub async fn select_menu_ids_by_role_id(db: &PgPool, role_id: i32) -> AppResult<Vec<i32>> {
    info!("[SERVICE] Select menu list by role id:{}", role_id);
    let menu_ids = sqlx::query!(
        "select menu_id from  sys_role_menu  where  role_id = $1",
        role_id
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|record| record.menu_id)
    .collect();
    Ok(menu_ids)
}

///根据条件分页查询角色列表
pub(crate) async fn page_role(
    db: &'static PgPool,
    query_page: ListRoleQuery,
) -> AppResult<PageReponse<SysRole>> {
    info!("[SERVICE] Entering page_role with query: {:?}", query_page);
    //处理时间条件
    let start_time = match query_page.begin_time {
        Some(s) => time::flexible_parse_datetime(s.as_str())?,
        None => None,
    };
    let end_time = match query_page.end_time {
        Some(s) => time::flexible_parse_datetime(s.as_str())?,
        None => None,
    };
    // //构建条件
    let mut sql_builder = SqlBuilder::for_pagination(db, "*", "sys_role", Some("del_flag  = '0'"));
    sql_builder
        .where_like("role_key", query_page.role_key.as_deref())
        .where_like("role_name", query_page.role_name.as_deref())
        .where_eq("status", query_page.status)
        .where_ge("create_time", start_time)
        .where_le("create_time", end_time);

    // 处理分页
    let mut page = 1;
    let mut page_size = 10;
    if let Some(pr) = query_page.page {
        page = pr.page;
        page_size = pr.page_size;
        sql_builder.paginate(pr.page, pr.page_size);
    }

    // 查询总数
    let count = sql_builder.count().await?;
    info!("[SERVICE]  Role cost count: {:?}", count);

    //查询列表
    let list: Vec<SysRole> = sql_builder.fetch_all().await?;
    info!("[SERVICE] Page role list: {:?}", list);
    Ok(PageReponse::new(list, page, page_size, count))
}

///修改角色状态
pub(crate) async fn change_status(db: &PgPool, role_id: u32, status: String) -> AppResult<u64> {
    info!(
        "[SERVICE] Entering change_status with role_id: {}, status: {}",
        role_id, status
    );
    let result = sqlx::query!(
            "UPDATE sys_role SET status = $1, update_by = 'admin', update_time = NOW() WHERE role_id = $2",
            status,
            role_id as i32
        ).execute(db)
            .await?;
    Ok(result.rows_affected())
}

//单元测试
#[cfg(test)]
mod user_test {

    use common::{AppResult, page_reqest::PageRequest};
    use framework::db::DBPool;
    use sqlx::{Pool, Postgres};

    use crate::role::model::ListRoleQuery;
    async fn get_db_test() -> AppResult<&'static Pool<Postgres>> {
        let url = "postgres://postgres:Sky%402024@172.16.100.200:25432/project?options=-c%20search_path%3Dsky_website";
        let db = DBPool::inint(url).await?;
        Ok(db)
    }

    //新增角色测试
    #[tokio::test]
    async fn select_by_id_test() -> anyhow::Result<()> {
        let db = get_db_test().await?;
        let role = super::select_by_id(db, 1).await?;
        println!("{role:?}");
        Ok(())
    }

    //分页查询角色测试
    #[tokio::test]
    async fn page_role_test() -> anyhow::Result<()> {
        let db = get_db_test().await?;
        let query = ListRoleQuery {
            role_name: Some("超级".to_string()),
            role_key: Some("admin".to_string()),
            status: Some('1'.to_string()),
            begin_time: Some("2023-01-01 00:00:00".to_string()),
            end_time: Some("2026-01-02 00:00:00".to_string()),
            page: Some(PageRequest {
                page: 1,
                page_size: 10,
            }),
        };
        let role = super::page_role(db, query).await?;
        println!("{role:?}");
        assert_eq!(1, 2);
        Ok(())
    }
}
