use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use sqlx::{postgres::PgConnectOptions, Executor, PgPool};

#[derive(Deserialize)]
pub struct Migrate {
    pub dsn: String,
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
        let dsn_options = self.dsn.parse::<PgConnectOptions>().unwrap();
        let db_name = dsn_options.get_database().unwrap().to_owned();
        let pg_options = dsn_options.database("postgres");

        let pool = PgPool::connect_with(pg_options).await.unwrap();
        let query = format!("create database {};", db_name);
        let mut conn = pool.acquire().await.unwrap();
        let _ = conn.execute(query.as_str()).await;

        drop(pool);

        let pool = PgPool::connect(&self.dsn).await.unwrap();

        sqlx::migrate!()
            .set_locking(false)
            .run(&pool)
            .await
            .unwrap();

        Ok(())
    }
}
