use std::time::Duration;

use common::{AppError, AppResult};
use sqlx::{Database, Pool, Postgres, pool::PoolOptions};
use tokio::sync::OnceCell;
use tracing::info;

static DBPOOLONCELOCK: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub struct DBPool;
impl DBPool {
    pub async fn inint(url: &str) -> AppResult<&Pool<Postgres>> {
        let pool = DBPOOLONCELOCK
            .get_or_init(|| async {
                create_db_pool::<Postgres>(url)
                    .await
                    .expect("数据库初始化失败")
            })
            .await;
        Ok(pool)
    }
    pub async fn get() -> AppResult<&'static Pool<Postgres>> {
        let pool = DBPOOLONCELOCK
            .get()
            .ok_or(AppError::Other("获取数据库链接失败".to_string()))?;
        Ok(pool)
    }
}

async fn create_db_pool<DB: Database>(database_url: &str) -> Result<Pool<DB>, sqlx::Error> {
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
        let setting = Setting::init()?;
        let pool = create_db_pool::<Postgres>(&setting.database.get_url()).await?;
        let isc = pool.is_closed();
        assert!(!isc);
        Ok(())
    }
}
