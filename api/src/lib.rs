mod command;
mod error;
mod group;
mod openapi;
mod projection;

use actix_files::NamedFile;
pub use openapi::ApiDoc;

use actix::{Actor, Addr};
use actix_jwks::JwksClient;
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    get,
    http::header::{self, HeaderValue, HttpDate, TryIntoHeaderValue},
    web::{self, Data},
    App as ActixApp, HttpServer,
};
use command::Command;
use evento::{EventStore, PgEngine};
use projection::Projection;
use pulsar::{Producer, Pulsar, TokioExecutor};
use serde::Deserialize;
use sqlx::PgPool;
use std::{path::PathBuf, sync::Arc, time::SystemTime};
use tokio::sync::Mutex;
use tracing::error;
use utoipa::{openapi::Server, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub zone: String,
    pub cmd: Addr<Command>,
    pub store: EventStore<PgEngine>,
    pub db: PgPool,
    pub group_producer: Arc<Mutex<Producer<TokioExecutor>>>,
}

#[derive(Deserialize, Clone)]
pub struct JwksOptions {
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct PikavOptions {
    pub url: String,
    pub namespace: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct PulsarOptions {
    pub url: String,
    pub namespace: String,
}

#[derive(Deserialize, Clone)]
pub struct OpenApiOptions {
    pub servers: Option<Vec<Server>>,
}

#[derive(Deserialize, Clone)]
pub struct SwaggerUIOptions {
    pub url: String,
}

pub struct AppOptions {
    pub zone: String,
    pub listen: String,
    pub jwks: JwksOptions,
    pub pikav: PikavOptions,
    pub dsn: String,
    pub pulsar: PulsarOptions,
    pub openapi: OpenApiOptions,
    pub swagger_ui: SwaggerUIOptions,
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

        let pool = match PgPool::connect(&self.options.dsn).await {
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

        let projection = Projection {
            db: &pool,
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

        let mut openapi = openapi::ApiDoc::openapi();
        openapi.servers = self.options.openapi.servers.clone();

        let swagger_ui_url = self.options.swagger_ui.url.to_owned();

        HttpServer::new(move || {
            ActixApp::new()
                .app_data(web::Data::new(AppState {
                    zone: zone.to_owned(),
                    cmd: cmd.clone(),
                    store: PgEngine::new(pool.clone()),
                    group_producer: group_producer.clone(),
                    db: pool.clone(),
                }))
                .app_data(Data::new(jwks_client.clone()))
                .app_data(Data::new(openapi.clone()))
                .service(web::scope("/api").service(group::scope()))
                .service(openapi::service)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url(swagger_ui_url.to_owned(), openapi.clone()),
                )
                .service(actix_files::Files::new("/static", "./static"))
                .service(
                    actix_files::Files::new("/", "./t1q69LzMP0I9")
                        .prefer_utf8(true)
                        .default_handler(fn_service(|req: ServiceRequest| async {
                            let (req, _) = req.into_parts();
                            let file = NamedFile::open_async("./static/index.html").await?;
                            let mut res = file.into_response(&req);

                            let headers = res.headers_mut();

                            headers.insert(
                                header::EXPIRES,
                                HeaderValue::from_static("Tue, 03 Jul 2001 06:00:00 GMT"),
                            );

                            headers.insert(
                                header::LAST_MODIFIED,
                                HttpDate::from(SystemTime::now()).try_into_value()?,
                            );

                            headers.insert(
                                header::CACHE_CONTROL,
                                HeaderValue::from_static(
                                    "max-age=0, no-cache, must-revalidate, proxy-revalidate",
                                ),
                            );

                            Ok(ServiceResponse::new(req, res))
                        })),
                )
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

async fn create_pulsar(options: &AppOptions) -> Result<Pulsar<TokioExecutor>, pulsar::Error> {
    Pulsar::builder(&options.pulsar.url, TokioExecutor)
        .build()
        .await
}
#[get("")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./files/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
