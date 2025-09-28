use crate::role::model::{ListRoleQuery, RoleDTO, SysRole};
use crate::role::service;
use common::page_reponse::PageReponse;
use common::{AppResult, response::ResponseResult};
use framework::db::DBPool;
use salvo::Writer;
use salvo::oapi::extract::PathParam;
use salvo::{handler, oapi::extract::JsonBody};
use serde_json::{Value, json};
use tracing::info;

#[handler]
pub async fn add(role: JsonBody<RoleDTO>) -> AppResult<ResponseResult<()>> {
    let role = role.into_inner();
    info!("[HANDLER] Entering role::insert  with body: {:?}", role);
    let db = DBPool::get().await?;
    service::add_role(db, role).await?;
    ResponseResult::success_msg("新增成功").into()
}

#[handler]
pub async fn delete(role_id: PathParam<i32>) -> AppResult<ResponseResult<()>> {
    info!(
        "[HANDLER] Entering role::delete  with role_id: {:?}",
        role_id
    );
    let db = DBPool::get().await?;
    service::delete_role(db, role_id.into_inner()).await?;
    ResponseResult::success_msg("删除成功").into()
}

#[handler]
pub async fn update(role: JsonBody<RoleDTO>) -> AppResult<ResponseResult<()>> {
    let role = role.into_inner();
    info!("[HANDLER] Entering role::update  with body: {:?}", role);
    if role.role_id.is_none() {
        return Err(common::AppError::Other("角色ID不能为空".to_string()));
    }
    let db = DBPool::get().await?;
    service::update_role(db, role).await?;
    ResponseResult::success_msg("修改成功").into()
}

#[handler]
pub async fn get_detail(role_id: PathParam<i32>) -> AppResult<ResponseResult<Value>> {
    let role_id = role_id.into_inner();
    info!("[HANDLER] Entering role::get  with role_id: {:?}", role_id);
    let db = DBPool::get().await?;
    //1. 先查询角色基本信息
    let role = service::select_by_id(db, role_id).await?;
    //2. 再查询角色对应的菜单列表
    let menu_ids = service::select_menu_ids_by_role_id(db, role_id).await?;
    let data = json!({
         "role": role,
         "menu_ids": menu_ids
    });
    ResponseResult::success(data).into()
}

#[handler]
pub async fn page(
    query_page: JsonBody<ListRoleQuery>,
) -> AppResult<ResponseResult<PageReponse<SysRole>>> {
    let query_page = query_page.into_inner();
    info!(
        "[HANDLER] Entering role::page  with query: {:?}",
        query_page
    );
    let db = DBPool::get().await?;
    let page_data = service::page_role(db, query_page).await?;
    ResponseResult::success(page_data).into()
}
