use salvo::Router;

pub mod dict;

pub fn init_router() -> Router {
    Router::new().path("/sys").push(dict::init_router())
}
