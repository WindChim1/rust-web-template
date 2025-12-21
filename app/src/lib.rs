use framework::midddleware::auth;
use salvo::{
    Router,
    oapi::OpenApi,
    prelude::{StaticFile, SwaggerUi},
};

pub fn init_router() -> Router {
    let router = Router::new()
        .hoop(system::handle::oper_log_middleware)
        .push(system::init_router())
        .push(
            Router::new()
                //需要认证的路由
                .hoop(auth)
                .push(monitor::init_router()),
        );
    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    let static_router = Router::with_path("admin").get(StaticFile::new("static/index.html"));

    router
        .push(static_router)
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
}
