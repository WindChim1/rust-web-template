use salvo::Router;

use crate::user::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("user")
        .push(Router::with_path("/login").get(handle::login))
        .push(Router::with_path("/refesh_token").get(handle::refresh_token_handler))
}
