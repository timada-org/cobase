use std::str::FromStr;

use cobase_api::{
    App, AppOptions, JwksOptions, OpenApiOptions, PikavOptions, PulsarOptions, SwaggerUIOptions,
};
use cobase_cluster::{Cluster, ClusterOptions};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct ServeAddr {
    pub api: String,
    pub cluster: String,
}

#[derive(Deserialize)]
pub struct Serve {
    pub zone: String,
    pub addr: ServeAddr,
    pub jwks: JwksOptions,
    pub pikav: PikavOptions,
    pub dsn: String,
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
            .add_source(Environment::with_prefix("cobase"))
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

        let cluster = Cluster::new(ClusterOptions {
            addr: self.addr.cluster.to_owned(),
        });

        let app = App::new(AppOptions {
            zone: self.zone.to_owned(),
            listen: self.addr.api.to_owned(),
            dsn: self.dsn.to_owned(),
            jwks: self.jwks.clone(),
            pikav: self.pikav.clone(),
            pulsar: self.pulsar.clone(),
            openapi: self.openapi.clone(),
            swagger_ui: self.swagger_ui.clone(),
        });

        actix_rt::spawn(async move { cluster.serve().await });

        app.run().await
    }
}
