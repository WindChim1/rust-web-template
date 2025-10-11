use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use common::{AppError, AppResult, page_reponse::PageReponse};
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use tracing::info;

use crate::user::model::{self, SysUser, SysUserDTO, SysUserVO};

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
    let password_hash = Argon2::default()
        .hash_password(
            sys_user_dto.password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )
        .map_err(|e| AppError::Other(e.to_string()))?
        .to_string();

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

pub(crate) async fn select_user_page(
    db: &PgPool,
    page_query: model::ListUserQuery,
) -> AppResult<PageReponse<SysUserVO>> {
    info!(
        "[SERVICE] Entering select_user_page with page_query: {:?}",
        page_query
    );

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT u.*, d.dept_name
         FROM sys_user u
         LEFT JOIN sys_dept d ON u.dept_id = d.dept_id
         WHERE u.del_flag = '0'",
    );

    let mut count_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT COUNT(*)
         FROM sys_user u
         LEFT JOIN sys_dept d ON u.dept_id = d.dept_id
         WHERE u.del_flag = '0'",
    );

    if let Some(name) = page_query.user_name {
        if !name.trim().is_empty() {
            let condition = format!("%{}%", name);
            query_builder
                .push(" AND u.user_name LIKE ")
                .push_bind(condition.clone());
            count_builder
                .push(" AND u.user_name LIKE ")
                .push_bind(condition);
        }
    }
    if let Some(phone) = page_query.phonenumber {
        if !phone.trim().is_empty() {
            let condition = format!("%{}%", phone);
            query_builder
                .push(" AND u.phonenumber LIKE ")
                .push_bind(condition.clone());
            count_builder
                .push(" AND u.phonenumber LIKE ")
                .push_bind(condition);
        }
    }
    if let Some(status) = page_query.status {
        if !status.trim().is_empty() {
            query_builder
                .push(" AND u.status = ")
                .push_bind(status.clone());
            count_builder.push(" AND u.status = ").push_bind(status);
        }
    }
    let total: (i64,) = count_builder.build_query_as().fetch_one(db).await?;

    // let page_num = page_query.page_num.unwrap_or(1);
    // let page_size = page_query.page_size.unwrap_or(10);
    // let offset = (page_num - 1) * page_size;
    // query_builder
    //     .push(" ORDER BY u.create_time DESC LIMIT ")
    //     .push_bind(page_size)
    //     .push(" OFFSET ")
    //     .push_bind(offset);
    todo!()
}
