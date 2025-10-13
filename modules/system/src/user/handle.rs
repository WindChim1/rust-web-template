use common::AppError;
use common::page_reponse::PageReponse;
use common::{AppResult, response::ResponseResult};
use framework::db::DBPool;
use salvo::Writer;
use salvo::handler;
use salvo::oapi::extract::PathParam;
use salvo::oapi::extract::{JsonBody, QueryParam};
use tracing::info;

use crate::role;
use crate::user::model::SysUserVO;
use crate::user::model::{SysUserDTO, UpdateUserDTO};
use crate::user::{self, model};

/// 添加用户
#[handler]
pub async fn add_user(user: JsonBody<SysUserDTO>) -> AppResult<ResponseResult<()>> {
    info!("[HANDLER_ADD] Entering clean 'add' handler.");
    let user = user.into_inner();
    let db = DBPool::get().await?;
    //1.控制user_name唯一性
    user::service::select_user_by_username(db, &user.user_name)
        .await
        .map(|o| o.map(|_| AppError::Other("用户名已存在".to_string())))?;

    //2.添加用户
    user::service::add_user(db, user).await?;
    ResponseResult::success_msg("添加成功").into()
}

/// 查看用户信息
#[handler]
pub async fn get_detail(user_id: PathParam<i32>) -> AppResult<ResponseResult<SysUserVO>> {
    let user_id = user_id.into_inner();
    info!(
        "[HANDLER] Entering user::get_detail with user_id: {}",
        user_id
    );
    let db = DBPool::get().await?;
    //1. 查询用户信息
    let mut user_vo = user::service::select_user_by_id(db, user_id).await?;
    //2. 查询关联的角色信息
    let role_list = role::service::select_role_list_by_user_id(db, user_id).await?;
    user_vo.role_list = Some(role_list);
    ResponseResult::success(user_vo).into()
}

/// 用户分页列表
#[handler]
pub async fn page(
    page_query: JsonBody<model::ListUserQuery>,
) -> AppResult<ResponseResult<PageReponse<SysUserVO>>> {
    info!("[HANDLER] Entering user::page_list.");
    let db = DBPool::get().await?;
    let mut user_list = user::service::select_user_page(db, page_query.into_inner()).await?;
    for user in user_list.items.iter_mut() {
        let role_list = role::service::select_role_list_by_user_id(db, user.user_id).await?;
        user.role_list = Some(role_list);
    }
    ResponseResult::success(user_list).into()
}

/// 修改用户密码
#[handler]
pub async fn reset_pwd(
    user_id: QueryParam<i32>,
    pwd: QueryParam<String>,
) -> AppResult<ResponseResult<()>> {
    let user_id: i32 = user_id.into_inner();
    let pwd = pwd.into_inner();

    info!(
        "[HANDLER] Entering user::update_pwd with user_id: {}, pwd: {}",
        user_id, pwd
    );
    let db = DBPool::get().await?;
    user::service::reset_user_password(db, user_id, &pwd).await?;
    ResponseResult::success_msg("修改密码成功").into()
}

/// 删除用户
#[handler]
pub async fn delete(user_id: PathParam<i32>) -> AppResult<ResponseResult<()>> {
    let user_id = user_id.into_inner();
    info!("[HANDLER] Entering user::delete with user_id: {}", user_id);
    let db = DBPool::get().await?;
    user::service::delete(db, user_id).await?;
    ResponseResult::success_msg("删除成功").into()
}

///修改用户
#[handler]
pub async fn update_user(user: JsonBody<UpdateUserDTO>) -> AppResult<ResponseResult<()>> {
    info!(
        "[HANDLER_UPDATE] Entering clean 'update' handler. user: {:?}",
        user
    );
    let user = user.into_inner();
    let db = DBPool::get().await?;

    //2.修改用户
    user::service::update_user(db, user).await?;
    ResponseResult::success_msg("修改成功").into()
}

/// 修改用户角色
#[handler]
pub async fn update_user_roles(
    user_id: QueryParam<i32>,
    role_ids: QueryParam<Vec<i32>>,
) -> AppResult<ResponseResult<()>> {
    let user_id = user_id.into_inner();
    let role_ids = role_ids.into_inner();
    info!(
        "[HANDLER_UPDATE] Entering clean 'update_user_roles' handler. user_id: {}, role_ids: {:?}",
        user_id, role_ids
    );
    let db = DBPool::get().await?;
    if !role_ids.is_empty() {
        user::service::update_user_roles(db, user_id, &role_ids).await?;
    }
    ResponseResult::success_msg("修改成功").into()
}
