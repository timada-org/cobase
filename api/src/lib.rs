mod command;
mod error;
mod group;
mod projection;

use actix::{Actor, Addr};
use actix_jwks::JwksClient;
use actix_web::{
    web::{self, Data},
    App as ActixApp, HttpServer,
};
use command::Command;
use evento::{EventStore, PgEngine};
use mongodb::{options::ClientOptions, Client};
use projection::Projection;
use pulsar::{Producer, Pulsar, TokioExecutor};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::log::error;

pub struct AppState {
    pub zone: String,
    pub cmd: Addr<Command>,
    pub store: EventStore<PgEngine>,
    pub read_db: mongodb::Database,
    pub group_producer: Arc<Mutex<Producer<TokioExecutor>>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwksOptions {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PikavOptions {
    pub url: String,
    pub namespace: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PulsarOptions {
    pub url: String,
    pub namespace: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseOptions {
    pub read: String,
    pub write: String,
}

pub struct AppOptions {
    pub zone: String,
    pub listen: String,
    pub jwks: JwksOptions,
    pub pikav: PikavOptions,
    pub database: DatabaseOptions,
    pub pulsar: PulsarOptions,
}

pub struct App {
    pub options: AppOptions,
}

impl App {
    pub fn new(options: AppOptions) -> Self {
        Self { options }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let zone = self.options.zone.to_owned();
        let jwks_client = JwksClient::new(&self.options.jwks.url);

        let pikva_client = pikav_client::Client::new(pikav_client::ClientOptions {
            url: self.options.pikav.url.to_owned(),
            namespace: self.options.pikav.namespace.to_owned(),
        });

        let pool = match PgPool::connect(&self.options.database.write).await {
            Ok(pool) => pool,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let pulsar = match create_pulsar(&self.options).await {
            Ok(pulsar) => pulsar,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let read_db = match create_read_database(&self.options).await {
            Ok(db) => db,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let projection = Projection {
            read_db: &read_db,
            pulsar: &pulsar,
            options: &self.options,
            pikav: &pikva_client,
        };

        if let Err(e) = group::start(&projection).await {
            error!("{e}");

            std::process::exit(1)
        }

        let group_producer = match create_producer("group", &pulsar, &self.options).await {
            Ok(producer) => producer,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let cmd = Command::new(pool.clone()).start();

        HttpServer::new(move || {
            ActixApp::new()
                .app_data(web::Data::new(AppState {
                    zone: zone.to_owned(),
                    cmd: cmd.clone(),
                    store: PgEngine::new(pool.clone()),
                    group_producer: group_producer.clone(),
                    read_db: read_db.clone(),
                }))
                .app_data(Data::new(jwks_client.clone()))
                .service(group::scope())
        })
        .bind(self.options.listen.to_owned())?
        .run()
        .await
    }
}

async fn create_producer(
    name: &str,
    pulsar: &Pulsar<TokioExecutor>,
    options: &AppOptions,
) -> Result<Arc<Mutex<Producer<TokioExecutor>>>, pulsar::Error> {
    pulsar
        .producer()
        .with_topic(format!("{}/{}", options.pulsar.namespace, name))
        .with_name(format!("cobase.{}", options.zone))
        .build()
        .await
        .map(|producer| Arc::new(Mutex::new(producer)))
}

async fn create_read_database(
    app_options: &AppOptions,
) -> Result<mongodb::Database, mongodb::error::Error> {
    let options = ClientOptions::parse(&app_options.database.read).await?;

    Client::with_options(options).map(|client| client.database("cobase"))
}

async fn create_pulsar(options: &AppOptions) -> Result<Pulsar<TokioExecutor>, pulsar::Error> {
    Pulsar::builder(&options.pulsar.url, TokioExecutor)
        .build()
        .await
}
