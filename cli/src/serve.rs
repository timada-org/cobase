use std::str::FromStr;

use api::{
    App, AppOptions, DatabaseOptions, JwksOptions, OpenApiOptions, PikavOptions, PulsarOptions,
    SwaggerUIOptions,
};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use tracing::Level;

#[derive(Deserialize)]
pub struct Serve {
    pub zone: String,
    pub listen: String,
    pub jwks: JwksOptions,
    pub pikav: PikavOptions,
    pub database: DatabaseOptions,
    pub pulsar: PulsarOptions,
    pub openapi: OpenApiOptions,
    pub swagger_ui: SwaggerUIOptions,
    pub log: Option<String>,
}

impl Serve {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path))
            .add_source(File::with_name(&format!("{}.local", path)).required(false))
            .add_source(Environment::with_prefix(env!("CARGO_PKG_NAME")))
            .build()?
            .try_deserialize()
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(
                self.log
                    .as_ref()
                    .map(|log| Level::from_str(log).expect("failed to deserialize log"))
                    .unwrap_or(Level::ERROR),
            )
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let mut database = self.database.clone();
        database.write = format!("{}/cobase?sslmode=disable", database.write);

        let app = App::new(AppOptions {
            zone: self.zone.to_owned(),
            listen: self.listen.to_owned(),
            jwks: self.jwks.clone(),
            pikav: self.pikav.clone(),
            pulsar: self.pulsar.clone(),
            openapi: self.openapi.clone(),
            swagger_ui: self.swagger_ui.clone(),
            database,
        });

        app.run().await
    }
}
