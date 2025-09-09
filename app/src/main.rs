use std::process;

use common::AppResult;
use framework::{config, db, log};
use salvo::prelude::*;
use sqlx::Postgres;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() -> AppResult<()> {
    tokio::select! {
        _= run()=>{
        }
        _= ctrl_c() =>{
            process::exit(0);
        }
    }
    Ok(())
}

async fn run() -> AppResult<()> {
    // Initialize logging subsystem
    log::init_tracing();
    // Initialize config subsystem
    let setting = config::Setting::init()?;
    let _db = db::create_db_pool::<Postgres>(&setting.database.get_url()).await?;
    let acceptor = TcpListener::new(("0.0.0.0", setting.server.port))
        .bind()
        .await;

    Server::new(acceptor).serve(app::init_router()).await;
    Ok(())
}
