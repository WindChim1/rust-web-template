use salvo::Router;

use crate::role::handle::*;
pub fn init_router() -> Router {
    Router::new()
        .path("role")
        .push(Router::with_path("add").post(add))
        .push(Router::with_path("delete/{role_id}").delete(delete))
        .push(Router::with_path("update").put(update))
        .push(Router::with_path("{role_id}").get(get_detail))
        .push(Router::with_path("page").get(page))
        .push(Router::with_path("change_status").get(change_status))
}
