mod command;
mod group;
mod openapi;
mod query;

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
use evento::{PgEngine, Publisher};
use query::Query;
use serde::Deserialize;
use sqlx::PgPool;
use std::{path::PathBuf, time::SystemTime};
use tracing::error;
use utoipa::{openapi::Server, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub cmd: Addr<Command>,
    pub query: Addr<Query>,
    pub publisher: Publisher<evento::store::PgEngine>,
}

#[derive(Deserialize, Clone)]
pub struct JwksOptions {
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct PikavOptions {
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
        let jwks_client = match JwksClient::new(&self.options.jwks.url).await {
            Ok(c) => c,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let pikva_client = match pikav_client::Client::new(pikav_client::ClientOptions {
            url: self.options.pikav.url.to_owned(),
            namespace: self.options.pikav.namespace.to_owned(),
        }) {
            Ok(pikva_client) => pikva_client,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let pool = match PgPool::connect(&self.options.dsn).await {
            Ok(pool) => pool,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let store = PgEngine::new(pool.clone());
        let query = Query::new(pool.clone()).start();
        let cmd = Command::new(store.clone()).start();
        let res = store
            .name(format!("cobase.{}", self.options.zone))
            .data(pool)
            .data(pikva_client.clone())
            .subscribe(group::projection::groups())
            .run()
            .await;

        let publisher = match res {
            Ok(p) => p,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let mut openapi = openapi::ApiDoc::openapi();
        openapi.servers = self.options.openapi.servers.clone();

        let swagger_ui_url = self.options.swagger_ui.url.to_owned();

        HttpServer::new(move || {
            ActixApp::new()
                .app_data(web::Data::new(AppState {
                    cmd: cmd.clone(),
                    query: query.clone(),
                    publisher: publisher.clone(),
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

#[get("")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./files/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
