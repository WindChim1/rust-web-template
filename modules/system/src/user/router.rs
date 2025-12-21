use salvo::Router;

use crate::user::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("user")
        .push(Router::with_path("page").post(handle::page))
        .push(Router::with_path("add").post(handle::add_user))
        .push(Router::with_path("{user_id}").get(handle::get_detail))
        .push(Router::with_path("/delete/{user_id}").get(handle::delete))
        .push(Router::with_path("update").post(handle::update_user))
}
