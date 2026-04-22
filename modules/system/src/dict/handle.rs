use common::AppResult;
use common::page_reponse::PageReponse;
use common::page_reqest::PageRequest;
use common::response::ResponseResult;
use framework::db::DBPool;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use tracing::info;

use crate::dict;
use crate::dict::model::{
    AddSysDictDataDTO, AddSysDictTypeDTO, SysDictData, SysDictType, SysDictTypePageQuery,
};
use salvo::Writer;

#[endpoint(tags("字典管理"), summary = "字典类型分页")]
pub async fn get_type_page(
    page_query: JsonBody<PageRequest<SysDictTypePageQuery>>,
) -> AppResult<ResponseResult<PageReponse<SysDictType>>> {
    info!("[HANDLER] Entering get type list");
    let db = DBPool::get().await?;
    let page = dict::service::dic_type_page(db, page_query.into_inner()).await?;
    ResponseResult::success(page).into()
}

#[endpoint(tags("字典管理"), summary = "根据字典类型分获字典list")]
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

#[endpoint(tags("字典管理"), summary = "添加字典类型")]
pub async fn add_dict_type(
    dict_type: JsonBody<AddSysDictTypeDTO>,
) -> AppResult<ResponseResult<u8>> {
    info!("[HANDLER] Entering add dict type: {:?}", dict_type);
    let db = DBPool::get().await?;
    let result = dict::service::add_dict_type(db, dict_type.into_inner()).await?;
    ResponseResult::success(result).into()
}
#[endpoint(tags("字典管理"), summary = "添加字典值")]
pub async fn add_dict_data(
    dict_data: JsonBody<AddSysDictDataDTO>,
) -> AppResult<ResponseResult<u8>> {
    info!("[HANDLER] Entering add dict data: {:?}", dict_data);
    let db = DBPool::get().await?;
    let result = dict::service::add_dict_data(db, dict_data.into_inner()).await?;
    ResponseResult::success(result).into()
}
