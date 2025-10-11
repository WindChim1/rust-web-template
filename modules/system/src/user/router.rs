use salvo::Router;

use crate::user::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("user")
        .push(Router::with_path("page").get(handle::page_list))
        .push(Router::with_path("add").get(handle::add_user))
        .push(Router::with_path("get_detail/{id}").get(handle::get_detail))
}
