use api::DatabaseOptions;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use sqlx::{Executor, PgPool};

#[derive(Deserialize)]
pub struct Migrate {
    pub database: DatabaseOptions,
}

impl Migrate {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path))
            .add_source(File::with_name(&format!("{}.local", path)).required(false))
            .add_source(Environment::with_prefix(env!("CARGO_PKG_NAME")))
            .build()?
            .try_deserialize()
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let pool = PgPool::connect(&self.database.write).await.unwrap();

        let mut conn = pool.acquire().await.unwrap();
        let _ = conn.execute("create database cobase;").await;

        drop(pool);

        let pool = PgPool::connect(&format!("{}/cobase", self.database.write))
            .await
            .unwrap();

        sqlx::migrate!()
            .set_locking(false)
            .run(&pool)
            .await
            .unwrap();

        Ok(())
    }
}
