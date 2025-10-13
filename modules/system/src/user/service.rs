use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use common::{AppError, AppResult, SqlBuilder, page_reponse::PageReponse};
use sqlx::{PgPool, Postgres, Transaction};
use tracing::info;

use crate::user::model::{self, SysUser, SysUserDTO, SysUserVO, UpdateUserDTO};

pub async fn select_user_by_username(db: &PgPool, user_name: &str) -> AppResult<Option<SysUser>> {
    info!(
        "[SERVICE] Entering select_user_by_username with user_name: '{}'",
        user_name
    );

    let user = sqlx::query_as!(
        SysUser,
        "SELECT * FROM sys_user WHERE user_name = $1 AND del_flag = '0'",
        user_name,
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}

/// 新增用户，并处理其与角色的关联关系（事务性）
pub async fn add_user(db: &PgPool, sys_user_dto: SysUserDTO) -> AppResult<u64, AppError> {
    info!("[SERVICE] Entering add_user with vo: {:?}", sys_user_dto);
    let mut tx = db.begin().await?;
    let password_hash = hash_password(&sys_user_dto.password)?;
    // 1. 插入用户基本信息
    let result= sqlx::query!(
        "INSERT INTO sys_user (user_name, nick_name, password, phone_number, email,  status, remark, create_by, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, 'admin', NOW()) RETURNING user_id",
        sys_user_dto.user_name,
        sys_user_dto.nick_name,
        password_hash,
        sys_user_dto.phone_number,
        sys_user_dto.email,
        sys_user_dto.status,
        sys_user_dto.remark
    )
    .fetch_one(&mut *tx)
    .await?;
    let user_id = result.user_id;

    info!("[TX] Inserted into sys_user, new user_id: {}", user_id);

    // 2. 插入用户和角色的关联信息
    if let Some(role_ids) = sys_user_dto.role_ids.as_ref().filter(|ids| !ids.is_empty()) {
        insert_user_role(&mut tx, user_id, role_ids).await?;
    }

    // 提交事务
    tx.commit().await?;
    info!(
        "[TX] Transaction committed successfully for user_id: {}",
        user_id
    );
    Ok(1)
}

/// 插入用户和角色的关联信息
pub async fn insert_user_role(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    role_ids: &[i32],
) -> Result<(), AppError> {
    info!(
        "[SERVICE] Inserting {} role associations for user_id: {}",
        role_ids.len(),
        user_id
    );
    let mut sql = "INSERT INTO sys_user_role (user_id, role_id) VALUES ".to_string();
    sql.push_str(
        &role_ids
            .iter()
            .map(|role_id| format!("({}, {})", user_id, role_id))
            .collect::<Vec<_>>()
            .join(", "),
    );
    sqlx::query(&sql).execute(&mut **tx).await?;
    info!("[SERVICE] Successfully inserted role associations.");
    Ok(())
}
/// 根据用户ID查询用户信息
pub async fn select_user_by_id(db: &PgPool, user_id: i32) -> AppResult<SysUserVO> {
    info!(
        "[SERVICE] Select user informaton  by fe user_id: {}",
        user_id
    );
    sqlx::query_as!(
        SysUser,
        "select * from  sys_user where user_id = $1",
        user_id
    )
    .fetch_one(db)
    .await
    .map(SysUserVO::from)
    .map_err(AppError::from)
}

/// 分页查询用户列表
pub(crate) async fn select_user_page(
    db: &'static PgPool,
    page_query: model::ListUserQuery,
) -> AppResult<PageReponse<SysUserVO>> {
    info!(
        "[SERVICE] Entering select_user_page with page_query: {:?}",
        page_query
    );
    let (page, page_size) = page_query
        .page
        .as_ref()
        .map(|p| (p.page, p.page_size))
        .unwrap_or((1, 10));

    let mut sql_builder = SqlBuilder::for_pagination(db, "*", "sys_user", Some("del_flag = '0' "));
    sql_builder
        .where_like("user_name", page_query.user_name.as_deref())
        .where_like("phone_number", page_query.phone_number.as_deref())
        .where_eq("status", page_query.status)
        .where_le("create_time", page_query.begin_time)
        .where_ge("create_time", page_query.end_time)
        .paginate(page, page_size);
    let count = sql_builder.count().await?;
    let users: Vec<SysUser> = sql_builder.fetch_all().await?;
    let users = users.into_iter().map(|u| u.into()).collect();

    Ok(PageReponse::new(users, page, page_size, count))
}

/// 修改用户状态
pub async fn change_user_status(
    db: &PgPool,
    user_id: i32,
    status: &str,
) -> AppResult<u64, AppError> {
    info!(
        "[SERVICE] Changing status for user_id: {} to status: {}",
        user_id, status
    );
    let resutl = sqlx::query!(
            "UPDATE sys_user SET status = $1, update_by = 'admin', update_time = NOW() WHERE user_id = $2",
            status,
            user_id
        ).execute(db).await?;
    info!(
        "[SERVICE] Updated status for user_id: {}. Rows affected: {}",
        user_id,
        resutl.rows_affected()
    );
    Ok(resutl.rows_affected())
}

/// 重置用户密码
pub async fn reset_user_password(
    db: &PgPool,
    user_id: i32,
    new_password: &str,
) -> AppResult<u64, AppError> {
    info!("[SERVICE] Resetting password for user_id: {}", user_id);
    let password_hash = hash_password(new_password)?;
    let result = sqlx::query!(
        "UPDATE sys_user SET password = $1, update_by = 'admin', update_time = NOW() WHERE user_id = $2",
        password_hash,
        user_id
    )
    .execute(db)
    .await?;
    info!(
        "[SERVICE] Password reset for user_id: {}. Rows affected: {}",
        user_id,
        result.rows_affected()
    );
    Ok(result.rows_affected())
}

/// 使用 Argon2 算法对密码进行哈希处理
fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|e| AppError::Other(e.to_string()))
        .map(|hash| hash.to_string())
}

/// 根据用户ID获取其角色标识列表
pub async fn get_user_roles(db: &PgPool, user_id: i32) -> AppResult<Vec<String>> {
    info!(
        "[HANDLER] Entering user::get_user_roles with user_id: {}",
        user_id
    );
    let roles = if user_id == 1 {
        // 如果是超级管理员
        vec!["admin".to_string()]
    } else {
        sqlx::query_scalar(
            "select sr.role_key from  sys_role  sr 
             left join  sys_user_role sur  on sr.role_id = sur.role_id
                   where  sr.status = '0' and sr.del_flag = '0' and sur.user_id = $1",
        )
        .bind(user_id)
        .fetch_all(db)
        .await?
        // sqlx::query(sql).bind(user_id).fetch_all(db).await?
    };
    info!("[HANDLER] User roles for user_id {}: {:?}", user_id, roles);

    Ok(roles)
}

pub async fn get_user_permissions(db: &PgPool, user_id: i32) -> AppResult<Vec<String>> {
    info!(
        "[HANDLER] Entering user::get_user_permissions with user_id: {}",
        user_id
    );
    let permissions = if user_id == 1 {
        // 如果是超级管理员
        sqlx::query_scalar("select perm from sys_menu where status = '0' and del_flag = '0' and perms is not null and perms != ''")
            .fetch_all(db)
            .await?
    } else {
        let sql = r#"
            select distinct sm.perms from sys_menu sm
            left join sys_role_menu srm on sm.menu_id = srm.menu_id
            left join sys_user_role sur on srm.role_id = sur.role_id
            left join sys_role sr on sur.role_id = sr.role_id
            where sr.status = '0' and sr.del_flag = '0' and sm.status = '0' and sm.del_flag = '0' and sur.user_id = $1 and sm.perms is not null and sm.perms  != ''
        "#;

        sqlx::query_scalar(sql).bind(user_id).fetch_all(db).await?
    };
    info!(
        "[HANDLER] User permissions for user_id {}: {:?}",
        user_id, permissions
    );

    Ok(permissions)
}

///删除用户（逻辑删除）
pub(crate) async fn delete(db: &PgPool, user_id: i32) -> AppResult<u64> {
    info!("[SERVICE] Deleting user with user_id: {}", user_id);
    sqlx::query!("UPDATE sys_user SET del_flag = '1', update_by = 'admin', update_time = NOW() WHERE user_id = $1", user_id)
        .execute(db)
        .await
        .map(|res| {
            info!(
                "[SERVICE] User with user_id: {} marked as deleted. Rows affected: {}",
                user_id,
                res.rows_affected()
            );
            res.rows_affected()
        }).map_err(AppError::from)
}

/// 测试用例
#[cfg(test)]
mod user_test {
    use framework::db::DBPool;

    use crate::user::{model::SysUserDTO, service::add_user};
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let url = "postgres://postgres:Sky%402024@172.16.100.200:25432/project?options=-c%20search_path%3Dsky_website";
        let db = DBPool::inint(url).await?;
        let user = SysUserDTO {
            user_name: "admin".to_string(),
            nick_name: "admin".to_string(),
            user_type: None,
            email: None,
            phone_number: None,
            avatar: None,
            password: "admin".to_string(),
            status: None,
            remark: None,
            role_ids: Some(vec![1]),
        };
        let recode = add_user(db, user).await?;
        assert_eq!(recode, 1);

        Ok(())
    }
}

/// 修改用户信息
pub(crate) async fn update_user(db: &PgPool, user: UpdateUserDTO) -> AppResult<u64> {
    info!("[SERVICE] Updating user with data: {:?}", user);
    let mut tx = db.begin().await?;
    //修改用户信息
    let resutl = sqlx::query!("update sys_user set nick_name = $1, phone_number = $2, email = $3, status = $4, remark = $5, update_by = 'admin', update_time = NOW() where user_id = $6",
            user.nick_name,
            user.phone_number,
            user.email,
            user.status,
            user.remark,
            user.user_id
         ).execute(&mut *tx).await?;
    //修改角色信息
    if let Some(role_ids) = user.role_ids
        && !role_ids.is_empty()
    {
        sqlx::query!("delete from sys_user_role where user_id = $1", user.user_id)
            .execute(&mut *tx)
            .await?;
        insert_user_role(&mut tx, user.user_id, &role_ids).await?;
    }
    tx.commit().await?;

    Ok(resutl.rows_affected())
}

pub(crate) async fn update_user_roles(
    db: &PgPool,
    user_id: i32,
    role_ids: &[i32],
) -> AppResult<()> {
    info!(
        "[SERVICE] Updating roles for user_id: {} with roles: {:?}",
        user_id, role_ids
    );
    let mut tx = db.begin().await?;
    sqlx::query!("delete from sys_user_role where user_id = $1", user_id)
        .execute(&mut *tx)
        .await?;
    insert_user_role(&mut tx, user_id, role_ids).await?;
    tx.commit().await.map_err(AppError::from)
}
