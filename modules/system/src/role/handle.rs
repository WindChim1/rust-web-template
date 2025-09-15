use crate::role::model::RoleDTO;
use crate::role::service;
use common::{AppResult, response::ResponseResult};
use framework::db::DBPool;
use salvo::Writer;
use salvo::{handler, oapi::extract::JsonBody};
use tracing::info;

#[handler]
pub async fn add(role: JsonBody<RoleDTO>) -> AppResult<ResponseResult<()>> {
    let role = role.into_inner();
    info!("[HANDLER] Entering role::insert  with body: {:?}", role);
    let db = DBPool::get().await?;
    service::add_role(db, role).await?;
    ResponseResult::success_msg("新增成功").into()
}
