use common::{AppResult, SqlBuilder, page_reponse::PageReponse, page_reqest::PageRequest};
use sqlx::PgPool;
use tracing::info;

use crate::dict::model::{
    AddSysDictDataDTO, AddSysDictTypeDTO, SysDictData, SysDictType, SysDictTypePageQuery,
};

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

pub(crate) async fn add_dict_type(
    db: &'static PgPool,
    dict_type: AddSysDictTypeDTO,
) -> AppResult<u8> {
    info!("[SERVICE] Entering add dict type: {:?}", dict_type);
    let result = sqlx::query!("insert into sys_dict_type (dict_name, dict_type, status, create_by, remark) values ($1, $2, $3, $4, $5)",  dict_type.dict_name, dict_type.dict_type, dict_type.status, dict_type.create_by, dict_type.remark)
        .execute(db).await?;
    Ok(result.rows_affected() as u8)
}

pub(crate) async fn add_dict_data(
    db: &'static PgPool,
    dict_data: AddSysDictDataDTO,
) -> AppResult<u8> {
    info!("[SERVICE] Entering add dict data: {:?}", dict_data);
    let result = sqlx::query!("insert into sys_dict_data (dict_sort, dict_label, dict_value, dict_type, is_default, status, create_by, remark) values ($1, $2, $3, $4, $5, $6, $7, $8)",   dict_data.dict_sort, dict_data.dict_label, dict_data.dict_value, dict_data.dict_type, dict_data.is_default, dict_data.status, dict_data.create_by, dict_data.remark)
        .execute(db).await?;
    Ok(result.rows_affected() as u8)
}
