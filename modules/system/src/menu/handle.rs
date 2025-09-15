use common::{AppResult, response::ResponseResult};
use framework::db::DBPool;
use salvo::{handler, oapi::extract::JsonBody};
use tracing::info;

use crate::menu::{model::MenuDTO, service};
use salvo::Writer;

#[handler]
pub async fn add(menu: JsonBody<MenuDTO>) -> AppResult<ResponseResult<()>> {
    let menu = menu.into_inner();
    info!("[HANDLER] Entering menu::insert with body: {:?}", menu);

    let db = DBPool::get().await?;
    service::add_menu(db, menu).await?;
    ResponseResult::success_msg("新增成功").into()
}
