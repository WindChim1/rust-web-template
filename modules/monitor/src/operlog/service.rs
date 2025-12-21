use common::{AppResult, SqlBuilder, page_reponse::PageReponse, page_reqest::PageRequest};
use sqlx::PgPool;
use tracing::info;

use crate::operlog::model::{ListOperLogQuery, OperLogDTO, OperLogVO, SysOperLog};

pub(crate) async fn page(
    db: &'static PgPool,
    params: PageRequest<ListOperLogQuery>,
) -> AppResult<PageReponse<OperLogVO>> {
    info!("[SERVICE] Entering operlog::page with query: {:?}", params);

    let mut sql_builder = SqlBuilder::for_pagination(db, "*", "sys_oper_log", None);
    sql_builder
        .where_like("title", params.query.title.as_deref())
        .where_like("oper_name", params.query.oper_name.as_deref())
        .where_like("oper_nick_name", params.query.oper_nick_name.as_deref())
        .where_ge("oper_time", params.query.start_time)
        .where_le("oper_time", params.query.end_time)
        .where_eq("status", params.query.status)
        .where_eq("business_type", params.query.business_type)
        .order_by("oper_time", Some("desc"))
        .paginate(params.page, params.page_size);

    let sql = sql_builder.quer_sql();
    info!("[SERVICE] Entering operlog::page with query: {:?}", sql);

    let oper_log_list: Vec<SysOperLog> = sql_builder.fetch_all().await?;
    let count = sql_builder.count().await?;
    let oper_log_page = PageReponse::new(oper_log_list, params.page, params.page_size, count);

    Ok(oper_log_page.convert())
}

pub async fn add(db: &PgPool, log: OperLogDTO) -> AppResult<()> {
    info!("[SERVICE] Entering add operlog with data: {:?}", log);
    sqlx::query!(
            "INSERT INTO sys_oper_log (title, business_type, method, request_method,operator_type, oper_name, oper_nick_name,oper_url, oper_ip,oper_location, oper_param, json_result, status,error_msg, oper_time, cost_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,$14,NOW(), $15)",
            log.title,
            log.business_type,
            log.method,
            log.request_method,
            log.operator_type,
            log.oper_name,
            log.oper_nick_name,
            log.oper_url,
            log.oper_ip,
            log.oper_location,
            log.oper_param,
            log.json_result,
            log.status,
            log.error_msg,
            log.cost_time
        ).execute(db).await?;
    Ok(())
}
