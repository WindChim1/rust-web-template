use salvo::Router;

use crate::login_info::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("login_info")
        .push(Router::with_path("page").get(handle::page))
}
