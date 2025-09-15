use salvo::Router;

use crate::menu::handle::add;

pub fn init_router() -> Router {
    Router::new()
        .path("menu")
        .push(Router::with_path("add").post(add))
}
