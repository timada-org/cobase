use std::path::Path;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Any, PgPool,
};

#[derive(Deserialize)]
pub struct Reset {
    pub dsn: String,
}

impl Reset {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path))
            .add_source(File::with_name(&format!("{path}.local")).required(false))
            .add_source(Environment::with_prefix(env!("CARGO_PKG_NAME")))
            .build()?
            .try_deserialize()
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let dsn = self.dsn.replace("cockroach", "postgres");
        let exists = crate::retry_connect_errors(&dsn, Any::database_exists)
            .await
            .unwrap();

        if exists {
            Any::drop_database(&dsn).await.unwrap();
        }

        Any::create_database(&dsn).await.unwrap();

        let pool = PgPool::connect(&self.dsn).await.unwrap();

        Migrator::new(Path::new("./migrations"))
            .await
            .unwrap()
            .set_locking(false)
            .run(&pool)
            .await
            .unwrap();

        Ok(())
    }
}
