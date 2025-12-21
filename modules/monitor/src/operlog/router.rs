use salvo::Router;

use crate::operlog::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("operlog")
        .push(Router::with_path("page").post(handle::page))
}
