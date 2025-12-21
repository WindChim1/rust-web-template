use common::{
    AppResult, page_reponse::PageReponse, page_reqest::PageRequest, response::ResponseResult,
};
use framework::db::DBPool;
use salvo::Writer;
use salvo::oapi::{endpoint, extract::JsonBody};
use tracing::info;

use crate::login_info::{
    model::{ListLogininforQuery, SysLoginInfor},
    service,
};

#[endpoint(tags("登录日志"), summary = "分页")]
pub(crate) async fn page(
    query: JsonBody<PageRequest<ListLogininforQuery>>,
) -> AppResult<ResponseResult<PageReponse<SysLoginInfor>>> {
    let query = query.into_inner();
    info!("[HANDLER] Entering operlog::page:{:?}", query);
    let db = DBPool::get().await?;
    let page_result = service::select_logininfor_list(db, query).await?;
    Ok(ResponseResult::success(page_result))
}
