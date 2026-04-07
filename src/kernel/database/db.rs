use super::db_errors::DbError;
use crate::kernel::structures::DbConfig;
use deadpool_postgres::{ ManagerConfig, Pool, RecyclingMethod, Runtime };
use tokio_postgres::NoTls;

pub struct Database {
    pool: Pool,
    config: DbConfig,
}

impl Database {
    pub async fn init(&self) -> Result<(), DbError> {
        let client = self.client().await?;

        client.batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id BIGSERIAL PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                active BOOLEAN NOT NULL DEFAULT TRUE
            );
            "#
        ).await?;

        Ok(())
    }

    pub fn new(config: DbConfig) -> Result<Self, DbError> {
        let mut cfg = deadpool_postgres::Config::new();

        cfg.host = Some(config.db_host.clone());
        cfg.port = Some(config.db_port.clone());
        cfg.dbname = Some(config.db_name.clone());
        cfg.user = Some(config.db_user.clone());
        cfg.password = Some(config.db_password.clone());

        cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        Ok(Self { pool, config })
    }

    pub async fn client(&self) -> Result<deadpool_postgres::Client, DbError> {
        Ok(self.pool.get().await?)
    }
}
