use std::str::FromStr;

use cobase_api::{
    App, AppOptions, EventoOptions, JwksOptions, OpenApiOptions, PikavOptions, SwaggerUIOptions,
};
use cobase_cluster::{Cluster, ClusterOptions};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use tracing::Level;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

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
    pub openapi: OpenApiOptions,
    pub swagger_ui: SwaggerUIOptions,
    pub evento: EventoOptions,
    pub log: Option<String>,
    pub public_folder: Option<String>,
}

impl Serve {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path))
            .add_source(File::with_name(&format!("{path}.local")).required(false))
            .add_source(Environment::with_prefix("cobase"))
            .build()?
            .try_deserialize()
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let log = self
            .log
            .as_ref()
            .map(|log| Level::from_str(log).expect("failed to deserialize log"))
            .unwrap_or(Level::ERROR);

        let filter = filter::Targets::new()
            .with_target("evento", log)
            .with_target("cobase_api", log)
            .with_target("cobase_cluster", log);

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(filter)
            .init();

        let cluster = Cluster::new(ClusterOptions {
            addr: self.addr.cluster.to_owned(),
        });

        let app = App::new(AppOptions {
            zone: self.zone.to_owned(),
            listen: self.addr.api.to_owned(),
            dsn: self.dsn.to_owned(),
            jwks: self.jwks.clone(),
            pikav: self.pikav.clone(),
            openapi: self.openapi.clone(),
            swagger_ui: self.swagger_ui.clone(),
            evento: self.evento.clone(),
            public_folder: self.public_folder.clone(),
        });

        actix_rt::spawn(async move { cluster.serve().await });

        app.run().await
    }
}
