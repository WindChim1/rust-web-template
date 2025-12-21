use salvo::Router;

pub mod login_info;
pub mod operlog;
pub fn init_router() -> Router {
    Router::new()
        .path("monitor")
        .push(operlog::router::init_router())
        .push(login_info::router::init_router())
}
