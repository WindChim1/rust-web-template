use salvo::Router;

pub mod dict;
pub mod user;

pub fn init_router() -> Router {
    Router::new().path("/sys").push(dict::init_router())
}
