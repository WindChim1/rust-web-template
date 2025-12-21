use common::AppResult;
use common::page_reponse::PageReponse;
use common::page_reqest::PageRequest;
use common::response::ResponseResult;
use framework::db::DBPool;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use tracing::info;

use crate::dict;
use crate::dict::model::{SysDictData, SysDictType, SysDictTypePageQuery};
use salvo::Writer;

#[endpoint(tags("字典管理"))]
pub async fn get_type_page(
    page_query: JsonBody<PageRequest<SysDictTypePageQuery>>,
) -> AppResult<ResponseResult<PageReponse<SysDictType>>> {
    info!("[HANDLER] Entering get type list");
    let db = DBPool::get().await?;
    let page = dict::service::dic_type_page(db, page_query.into_inner()).await?;
    ResponseResult::success(page).into()
}

#[endpoint(tags("字典管理"))]
pub async fn get_data_list_by_type(
    dict_type: QueryParam<String>,
) -> AppResult<ResponseResult<Vec<SysDictData>>> {
    info!(
        "[HANDLER] Entering get data list by type id with type id: {}",
        dict_type
    );
    let db = DBPool::get().await?;
    let datas = dict::service::get_data_list_by_type(db, &dict_type).await?;
    ResponseResult::success(datas).into()
}
