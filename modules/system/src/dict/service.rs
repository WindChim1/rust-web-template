use common::{AppResult, SqlBuilder, page_reponse::PageReponse, page_reqest::PageRequest};
use sqlx::PgPool;
use tracing::info;

use crate::dict::model::{SysDictData, SysDictType, SysDictTypePageQuery};

pub(crate) async fn get_data_list_by_type(
    db: &PgPool,
    type_id: &str,
) -> AppResult<Vec<SysDictData>> {
    info!(
        "[SERVICE] Entering get data list by type id with type id: {}",
        type_id
    );

    let datas = sqlx::query_as!(
        SysDictData,
        r#"
         select * from sys_dict_data where dict_type = $1
         "#,
        type_id
    )
    .fetch_all(db)
    .await?;
    Ok(datas)
}

pub(crate) async fn dic_type_page(
    db: &'static PgPool,
    page_query: PageRequest<SysDictTypePageQuery>,
) -> AppResult<PageReponse<SysDictType>> {
    info!("[SERVICE] Entering get type page");
    let page = page_query.page;
    let page_size = page_query.page_size;
    let mut sql_builder = SqlBuilder::for_pagination(db, "*", "sys_dict_type", None);
    sql_builder
        .where_like("dict_name", page_query.query.dict_name.as_deref())
        .where_eq("dict_type", page_query.query.dict_type)
        .where_eq("status", page_query.query.status)
        .paginate(page, page_size);

    // 查询总数
    let count = sql_builder.count().await?;
    info!("[SERVICE]  dict type  count: {:?}", count);

    //查询列表
    let list: Vec<SysDictType> = sql_builder.fetch_all().await?;
    info!("[SERVICE] Page dict type  list: {:?}", list);
    Ok(PageReponse::new(list, page, page_size, count))
}
