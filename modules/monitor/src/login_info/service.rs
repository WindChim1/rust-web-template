use crate::login_info::model::ListLogininforQuery;

use super::model::SysLoginInfor;
use common::{
    AppResult, SqlBuilder, error::AppError, page_reponse::PageReponse, page_reqest::PageRequest,
};
use sqlx::PgPool;
use tracing::info;

/// 新增一条登录日志记录
pub async fn add_logininfor(db: &PgPool, log: SysLoginInfor) -> Result<(), AppError> {
    info!(
        "[SERVICE] Preparing to add login information log for user: {:?}",
        log.user_name
    );

    sqlx::query!(
        "INSERT INTO sys_login_infor (user_name, ipaddr, login_location, browser, os, status, msg, login_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        log.user_name,
        log.ipaddr,
        log.login_location,
        log.browser,
        log.os,
        log.status,
        log.msg,
        log.login_time,
    )
        .execute(db)
        .await?;

    info!("[SERVICE] Login information log added successfully.");
    Ok(())
}

/// 查询登录日志列表（分页）
pub async fn select_logininfor_list(
    db: &'static PgPool,
    params: PageRequest<ListLogininforQuery>,
) -> AppResult<PageReponse<SysLoginInfor>> {
    info!(
        "[SERVICE] Entering select_logininfor_list with params: {:?}",
        params
    );
    let login_info_page = SqlBuilder::for_pagination(db, "*", "sys_logininfor", None)
        .where_like("user_name", params.query.user_name.as_deref())
        .where_ge("login_time", params.query.start_time)
        .where_le("login_time", params.query.end_time)
        .where_eq("status", params.query.status)
        .order_by("login_time", Some("desc"))
        .fetch_paged(params.page, params.page_size)
        .await?;
    Ok(login_info_page)
}

// /// 批量删除登录日志
// pub async fn delete_logininfor_by_ids(db: &MySqlPool, info_ids: &[i64]) -> Result<u64, AppError> {
//     info!(
//         "[SERVICE] Entering delete_logininfor_by_ids with ids: {:?}",
//         info_ids
//     );
//     let params = info_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
//     let sql = format!("DELETE FROM sys_logininfor WHERE info_id IN ({})", params);

//     let mut query = sqlx::query(&sql);
//     for id in info_ids {
//         query = query.bind(id);
//     }

//     let result = query.execute(db).await?;
//     info!(
//         "[DB_RESULT] Deleted {} logininfors.",
//         result.rows_affected()
//     );
//     Ok(result.rows_affected())
// }

// /// 清空所有登录日志
// pub async fn clean_logininfor(db: &Postgres) -> Result<u64, AppError> {
//     info!("[SERVICE] Entering clean_logininfor");
//     let result = sqlx::query("TRUNCATE TABLE sys_logininfor")
//         .execute(db)
//         .await?;
//     info!("[DB_RESULT] Truncated sys_logininfor table.");
//     Ok(result.rows_affected())
// }
