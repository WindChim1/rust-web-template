use common::{AppError, AppResult, response::ResponseResult};
use framework::{
    db::DBPool,
    jwt::{CLAIMS, Claims},
};
use salvo::{
    Depot, handler,
    oapi::extract::{JsonBody, PathParam},
};
use tracing::info;

use crate::menu::{
    model::{MenuDTO, MenuTreeVo, RouterVo, SysMenu},
    service,
};
use salvo::Writer;

///新增菜单
#[handler]
pub async fn add(menu: JsonBody<MenuDTO>) -> AppResult<ResponseResult<()>> {
    let menu = menu.into_inner();
    info!("[HANDLER] Entering menu::insert with body: {:?}", menu);

    let db = DBPool::get().await?;
    service::add_menu(db, menu).await?;
    ResponseResult::success_msg("新增成功").into()
}

///修改菜单
#[handler]
pub async fn update(menu: JsonBody<MenuDTO>) -> AppResult<ResponseResult<()>> {
    let menu = menu.into_inner();
    info!("[HANDLER] Entering menu::insert with body: {:?}", menu);
    if menu.menu_id.is_none() {
        return Err(AppError::Other("菜单ID不能为空".to_string()));
    }

    let db = DBPool::get().await?;
    service::update_menu(db, menu).await?;
    ResponseResult::success_msg("修改成功").into()
}

/// 获取菜单树（根据权限）
#[handler]
pub async fn get_menu_tree(depot: &mut Depot) -> AppResult<ResponseResult<Vec<MenuTreeVo>>> {
    let user_id = depot
        .get::<Claims>(CLAIMS)
        .map(|s| &s.sub)
        .map_err(|_| AppError::Other("获取用户信息失败".to_string()))?;
    info!(
        "[HANDLER] Entering get menu tree  with user id in claims {:?}",
        user_id
    );
    let db = DBPool::get().await?;
    let menu_tree = service::select_menu_tree_by_user_id(db, *user_id).await?;

    ResponseResult::success(menu_tree).into()
}

///获取菜单列表
#[handler]
pub async fn list() -> AppResult<ResponseResult<Vec<MenuTreeVo>>> {
    info!("[HANDLER] Entering get menu tree  of all  menus");
    let menus = service::select_all_menu_list(DBPool::get().await?).await?;
    let menu_tree = MenuTreeVo::build_menu_tree(menus);
    ResponseResult::success(menu_tree).into()
}

//获取菜单详情信息
#[handler]
pub async fn get_detail(menu_id: PathParam<i32>) -> AppResult<ResponseResult<SysMenu>> {
    let menu = service::select_menu_by_id(DBPool::get().await?, menu_id.into_inner()).await?;
    ResponseResult::success(menu).into()
}

//删除菜单
#[handler]
pub async fn delete(menu_id: PathParam<i32>) -> AppResult<ResponseResult<()>> {
    service::delete_menu_by_id(DBPool::get().await?, menu_id.into_inner()).await?;
    ResponseResult::success_msg("删除成功").into()
}

///FIX:  完善功能(不确定是否需要)
#[handler]
pub async fn get_routers(depot: &mut Depot) -> AppResult<ResponseResult<Vec<RouterVo>>> {
    let user_id = depot
        .get::<Claims>(CLAIMS)
        .map(|s| &s.sub)
        .map_err(|_| AppError::Other("获取用户信息失败".to_string()))?;
    info!(
        "[HANDLER] Entering get routers  with user id in claims {:?}",
        user_id
    );
    let db = DBPool::get().await?;
    let menu_tree = service::select_menu_tree_by_user_id(db, *user_id).await?;

    let router_tree = RouterVo::build_from_menu_tree(menu_tree);
    ResponseResult::success(router_tree).into()
}
