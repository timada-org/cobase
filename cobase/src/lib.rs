pub mod command;
pub mod group;
pub mod query;

#[cfg(test)]
mod tests {
    use actix::Actor;
    use config::{Config, ConfigError, Environment, File};
    use evento::{Context, PgEngine};
    use serde::Deserialize;
    use sqlx::PgPool;
    use std::{path::PathBuf, time::Duration};

    use crate::{command::Command, query::Query};

    #[derive(Deserialize, Clone)]
    pub struct PikavOptions {
        pub url: String,
        pub namespace: String,
    }

    #[derive(Deserialize)]
    pub struct CobaseConfig {
        pub pikav: PikavOptions,
        pub dsn: String,
    }

    impl CobaseConfig {
        pub fn new(path: &str) -> Result<Self, ConfigError> {
            Config::builder()
                .add_source(File::with_name(path))
                .add_source(File::with_name(&format!("{path}.local")).required(false))
                .add_source(Environment::with_prefix("cobase"))
                .build()?
                .try_deserialize()
        }
    }

    pub(crate) async fn create_context(test_name: &str) -> Context {
        let config_path =
            std::env::var("TEST_CONFIG_PATH").unwrap_or("configs/default.yml".to_owned());

        let mut root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root_dir.pop();
        root_dir.push(config_path);

        let config = CobaseConfig::new(root_dir.to_str().unwrap()).unwrap();

        let pikav_client = pikav_client::Client::new(pikav_client::ClientOptions {
            url: config.pikav.url.to_owned(),
            namespace: config.pikav.namespace.to_owned(),
        })
        .unwrap();

        let pool = PgPool::connect(&config.dsn).await.unwrap();
        let evento = PgEngine::new(pool.clone())
            .name(format!("cobase.test.{test_name}"))
            .data(pool.clone())
            .data(pikav_client.clone())
            .subscribe(crate::group::projection::groups());
        let producer = evento.run_with_delay(Duration::from_secs(0)).await.unwrap();
        let command = Command::new(evento.clone(), producer).start();
        let query = Query::new(pool.clone()).start();

        let mut ctx = Context::new();
        ctx.insert(config);
        ctx.insert(pikav_client);
        ctx.insert(pool);
        ctx.insert(evento);
        ctx.insert(query);
        ctx.insert(command);

        ctx
    }
}
