use common::{
    AppResult, page_reponse::PageReponse, page_reqest::PageRequest, response::ResponseResult,
};
use framework::db::DBPool;
use salvo::prelude::*;
use salvo::{Writer, oapi::extract::JsonBody};
use tracing::info;

use crate::operlog::model::{ListOperLogQuery, OperLogVO};
use crate::operlog::service; // 引入上面定义的 LogMeta

#[endpoint(tags("操作日志"), summary = "分页")]
pub(crate) async fn page(
    query: JsonBody<PageRequest<ListOperLogQuery>>,
) -> AppResult<ResponseResult<PageReponse<OperLogVO>>> {
    let query = query.into_inner();
    info!("[HANDLER] Entering operlog::page:{:?}", query);
    let db = DBPool::get().await?;
    let page_result = service::page(db, query).await?;
    Ok(ResponseResult::success(page_result))
}
