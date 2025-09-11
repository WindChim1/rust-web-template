use common::AppResult;
use framework::{
    config, db,
    jwt::{JWTONCELOCK, JwtAuthUtil},
    log,
};
use salvo::prelude::*;
use sqlx::Postgres;

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
    JWTONCELOCK.get_or_init(|| JwtAuthUtil::new((&setting.jwt).into()));
    let _db = db::create_db_pool::<Postgres>(&setting.database.get_url()).await?;
    let acceptor = TcpListener::new(("0.0.0.0", setting.server.port))
        .bind()
        .await;

    Server::new(acceptor).serve(app::init_router()).await;
    Ok(())
}
