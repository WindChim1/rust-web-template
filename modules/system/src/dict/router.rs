use framework::midddleware::auth;
use salvo::Router;

use crate::dict::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("dict")
        .hoop(auth)
        .push(
            Router::new()
                .path("type")
                .push(Router::with_path("/list").get(handle::get_type_list)),
        )
        .push(
            Router::new()
                .path("data")
                .push(Router::with_path("/list").get(handle::get_data_list_by_type_id)),
        )
}
