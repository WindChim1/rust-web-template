use salvo::Router;

pub mod dict;
pub mod user;

pub fn init_router() -> Router {
    Router::new()
        .path("/sys")
        .push(user::router::init_router())
        .push(dict::init_router())
}
