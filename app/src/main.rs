use common::AppResult;
use framework::{config, db::DBPool, jwt::JWTTool, log};
use salvo::prelude::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    run().await
}

async fn run() -> AppResult<()> {
    // Initialize logging subsystem
    log::init_tracing();
    // Initialize config subsystem
    let setting = config::Setting::init()?;
    // Initialize jwt auth util
    JWTTool::init((&setting.jwt).into());
    // Initialize jwt auth util
    DBPool::inint(&setting.database.get_url()).await?;

    let acceptor = TcpListener::new(("0.0.0.0", setting.server.port))
        .bind()
        .await;

    Server::new(acceptor).serve(app::init_router()).await;
    Ok(())
}
