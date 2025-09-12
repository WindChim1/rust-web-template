use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use common::{AppError, AppResult};
use framework::db::DBPool;
use sqlx::{PgPool, Postgres, Transaction};
use tracing::info;

use crate::user::model::{SysUser, SysUserDTO};

pub async fn select_user_by_user_name(user_name: &str) -> AppResult<Option<SysUser>> {
    info!(
        "[SERVICE] Entering select_user_by_username with user_name: '{}'",
        user_name
    );
    let pool = DBPool::get().await?;

    let user = sqlx::query_as!(
        SysUser,
        "SELECT * FROM sys_user WHERE user_name = $1 AND del_flag = '0'",
        user_name,
    )
    .fetch_optional(pool)
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
    if let Some(role_ids) = sys_user_dto.role_ids {
        if !role_ids.is_empty() {
            insert_user_role(&mut tx, user_id as i64, &role_ids).await?;
        }
    }

    tx.commit().await?;
    info!(
        "[TX] Transaction committed successfully for user_id: {}",
        user_id
    );
    Ok(1)
}

async fn insert_user_role(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    role_ids: &[i64],
) -> Result<(), AppError> {
    info!(
        "[TX_HELPER] Inserting {} role associations for user_id: {}",
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
    info!("[TX_HELPER] Successfully inserted role associations.");
    Ok(())
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
