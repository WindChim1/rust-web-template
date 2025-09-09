use salvo::Router;

pub fn init_router() -> Router {
    Router::new().push(system::init_router())
}
