use salvo::Router;

use crate::dict::handle;

pub fn init_router() -> Router {
    Router::new()
        .path("dict")
        .push(
            Router::new()
                .path("type")
                .push(Router::with_path("/page").post(handle::get_type_page)),
        )
        .push(
            Router::new()
                .path("data")
                .push(Router::with_path("/list_by_type").get(handle::get_data_list_by_type)),
        )
}
