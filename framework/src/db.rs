use std::time::Duration;

use sqlx::{Database, Pool, pool::PoolOptions};
use tracing::info;

pub async fn create_db_pool<DB: Database>(database_url: &str) -> Result<Pool<DB>, sqlx::Error> {
    info!("creating dabata connection pool...");

    let pool = PoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .idle_timeout(Duration::from_secs(8))
        .connect(database_url)
        .await?;
    info!("✅  database connection pool created success!");
    Ok(pool)
}

#[cfg(test)]
mod db_test {
    use anyhow::{self, Ok};
    use sqlx::Postgres;

    use crate::{config::Setting, db::create_db_pool};
    #[tokio::test]
    async fn init_config_test() -> anyhow::Result<()> {
        let setting = Setting::new()?;
        let pool = create_db_pool::<Postgres>(&setting.database.get_url()).await?;
        let isc = pool.is_closed();
        assert!(!isc);
        Ok(())
    }
}
