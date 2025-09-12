use super::model::SysLoginInfor;
use common::error::AppError;
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

// /// 查询登录日志列表（分页）
// pub async fn select_logininfor_list(
//     db: &PgPool,
//     params: ListLogininforQuery,
// ) -> Result<TableDataInfo<SysLogininfor>, AppError> {
//     info!(
//         "[SERVICE] Entering select_logininfor_list with params: {:?}",
//         params
//     );

//     let mut sql = "SELECT * FROM sys_logininfor WHERE 1=1".to_string();

//     if let Some(ipaddr) = params.ipaddr {
//         if !ipaddr.trim().is_empty() {
//             sql.push_str(&format!(" AND ipaddr LIKE '%{}%'", ipaddr));
//         }
//     }
//     if let Some(user_name) = params.user_name {
//         if !user_name.trim().is_empty() {
//             sql.push_str(&format!(" AND user_name LIKE '%{}%'", user_name));
//         }
//     }
//     if let Some(status) = params.status {
//         if !status.trim().is_empty() {
//             sql.push_str(&format!(" AND status = '{}'", status));
//         }
//     }
//     if let Some(begin_time) = params.begin_time {
//         if !begin_time.trim().is_empty() {
//             sql.push_str(&format!(
//                 " AND date_format(login_time,'%y%m%d') >= date_format('{}','%y%m%d')",
//                 begin_time
//             ));
//         }
//     }
//     if let Some(end_time) = params.end_time {
//         if !end_time.trim().is_empty() {
//             sql.push_str(&format!(
//                 " AND date_format(login_time,'%y%m%d') <= date_format('{}','%y%m%d')",
//                 end_time
//             ));
//         }
//     }

//     let count_sql = format!("SELECT COUNT(*) as count FROM ({}) temp_table", sql);
//     let total: i64 = sqlx::query(&count_sql).fetch_one(db).await?.get("count");

//     let page_num = params.page_num.unwrap_or(1);
//     let page_size = params.page_size.unwrap_or(10);
//     let offset = (page_num - 1) * page_size;

//     sql.push_str(&format!(
//         " ORDER BY login_time DESC LIMIT {} OFFSET {}",
//         page_size, offset
//     ));

//     info!("[DB_QUERY] Executing query for logininfor list: {}", sql);
//     let rows: Vec<SysLogininfor> = sqlx::query_as(&sql).fetch_all(db).await?;
//     info!(
//         "[DB_RESULT] Found {} logininfors for the current page.",
//         rows.len()
//     );

//     Ok(TableDataInfo::new(rows, total))
// }

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
