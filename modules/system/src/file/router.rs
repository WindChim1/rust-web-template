use framework::midddleware::auth;
use salvo::Router;

use crate::file::handle::{get, index, upload};

pub fn init_router() -> Router {
    Router::new()
        .path("file")
        .push(Router::with_path("get").get(get))
        .push(
            Router::new()
                .hoop(auth)
                .push(Router::with_path("upload").post(upload))
                .push(Router::with_path("index").get(index)),
        )
}
