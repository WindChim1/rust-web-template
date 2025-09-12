use salvo::Router;

use crate::handle::{get_captcha_image, login};

pub mod dict;
pub mod handle;
pub mod role;
pub mod user;

pub fn init_router() -> Router {
    Router::new()
        //系统级别router
        .path("/sys")
        .push(
            Router::new()
                .push(Router::with_path("catpcha").get(get_captcha_image))
                .push(Router::with_path("login").post(login)),
        )
        //用户相关router
        .push(user::router::init_router())
        //枚举相关接口
        .push(dict::init_router())
}
