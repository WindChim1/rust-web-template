use framework::midddleware::auth;
use salvo::Router;

use crate::handle::{get_captcha_image, login, refresh_token_handler};

pub mod dict;
pub mod handle;
pub mod menu;
pub mod model;
pub mod role;
pub mod user;

pub fn init_router() -> Router {
    Router::new()
        .path("sys")
        //系统级别router
        .push(
            Router::new()
                //获取验证码
                .push(Router::with_path("catpcha").get(get_captcha_image))
                //登录
                .push(Router::with_path("login").post(login))
                //刷新token
                .push(Router::with_path("refresh_token").post(refresh_token_handler)),
        )
        .push(
            Router::new()
                //需要认证的路由
                .hoop(auth)
                //用户相关router
                .push(user::init_router())
                //枚举相关接口
                .push(dict::init_router())
                //菜单相关
                .push(menu::init_router())
                //角色相关
                .push(role::init_router()),
        )
}
