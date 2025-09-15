use common::{AppResult, response::ResponseResult};
use framework::db::DBPool;
use salvo::Writer;
use salvo::handler;
use salvo::oapi::extract::JsonBody;
use salvo::oapi::extract::PathParam;
use tracing::info;

use crate::role;
use crate::user;
use crate::user::model::SysUserDTO;
use crate::user::model::SysUserVO;

/// 添加用户
#[handler]
pub async fn add_user(user: JsonBody<SysUserDTO>) -> AppResult<ResponseResult<()>> {
    info!("[HANDLER_ADD] Entering clean 'add' handler.");
    let user = user.into_inner();
    let db = DBPool::get().await?;
    user::service::add_user(db, user).await?;
    ResponseResult::success_msg("添加成功").into()
}

/// 查看用户信息
#[handler]
pub async fn get_detail(id: PathParam<i32>) -> AppResult<ResponseResult<SysUserVO>> {
    let user_id = id.into_inner();
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
