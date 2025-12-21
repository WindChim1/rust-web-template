use common::AppError;
use common::page_reponse::PageReponse;
use common::page_reqest::PageRequest;
use common::{AppResult, response::ResponseResult};
use framework::db::DBPool;
use monitor::operlog::model::BusinessType;
use salvo::oapi::endpoint;
use salvo::oapi::extract::PathParam;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Writer};
use tracing::info;

use crate::handle::LogMeta;
use crate::role;
use crate::user::model::SysUserVO;
use crate::user::model::{SysUserAddDTO, SysUserUpdateDTO};
use crate::user::{self, model};

/// 添加用户
#[endpoint(tags("用户管理"))]
pub async fn add_user(
    user: JsonBody<SysUserAddDTO>,
    depot: &mut Depot,
) -> AppResult<ResponseResult<()>> {
    info!("[HANDLER_ADD] Entering clean 'add' handler.");

    //添加日志
    LogMeta::set(depot, "用户管理", BusinessType::Add.get_value(), "添加用户");
    let user = user.into_inner();
    let db = DBPool::get().await?;
    //1.控制user_name唯一性
    if user::service::select_user_by_username(db, &user.phone_number)
        .await?
        .is_some()
    {
        return Err(AppError::Other("手机号已存在".to_string()));
    }

    //2.添加用户
    user::service::add_user(db, user).await?;
    ResponseResult::success_msg("添加成功").into()
}

/// 查看用户信息
#[endpoint(tags("用户管理"), summary = "查看用户信息")]
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
#[endpoint(tags("用户管理"))]
pub async fn page(
    page_query: JsonBody<PageRequest<model::ListUserQuery>>,
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
#[endpoint(tags("用户管理"))]
pub async fn reset_pwd(
    user_id: QueryParam<i32>,
    pwd: QueryParam<String>,
    depot: &mut Depot,
) -> AppResult<ResponseResult<()>> {
    let user_id: i32 = user_id.into_inner();
    let pwd = pwd.into_inner();

    info!(
        "[HANDLER] Entering user::update_pwd with user_id: {}, pwd: {}",
        user_id, pwd
    );

    //添加日志
    LogMeta::set(
        depot,
        "用户管理",
        BusinessType::Update.get_value(),
        "修改密码",
    );
    let db = DBPool::get().await?;
    user::service::reset_user_password(db, user_id, &pwd).await?;
    ResponseResult::success_msg("修改密码成功").into()
}

/// 删除用户
#[endpoint(tags("用户管理"))]
pub async fn delete(user_id: PathParam<i32>, depot: &mut Depot) -> AppResult<ResponseResult<()>> {
    let user_id = user_id.into_inner();
    info!("[HANDLER] Entering user::delete with user_id: {}", user_id);
    //添加日志
    LogMeta::set(
        depot,
        "用户管理",
        BusinessType::Delete.get_value(),
        "删除用户",
    );
    let db = DBPool::get().await?;
    user::service::delete(db, user_id).await?;
    ResponseResult::success_msg("删除成功").into()
}

///修改用户
#[endpoint(tags("用户管理"))]
pub async fn update_user(
    user: JsonBody<SysUserUpdateDTO>,
    depot: &mut Depot,
) -> AppResult<ResponseResult<()>> {
    info!(
        "[HANDLER_UPDATE] Entering clean 'update' handler. user: {:?}",
        user
    );
    //添加日志
    LogMeta::set(
        depot,
        "用户管理",
        BusinessType::Update.get_value(),
        "修改用户",
    );
    let user = user.into_inner();
    let db = DBPool::get().await?;

    //2.修改用户
    user::service::update_user(db, user).await?;
    ResponseResult::success_msg("修改成功").into()
}

/// 修改用户角色
#[endpoint(tags("用户管理"))]
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
