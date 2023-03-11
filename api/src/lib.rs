mod group;
mod openapi;

use actix_files::NamedFile;
pub use openapi::ApiDoc;

use actix::{Actor, Addr};
use actix_jwks::JwksClient;
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    http::header::{self, HeaderValue, HttpDate, TryIntoHeaderValue},
    web::{self, Data},
    App as ActixApp, HttpServer,
};
use cobase::{command::Command, query::Query};
use evento::PgEngine;
use serde::Deserialize;
use sqlx::PgPool;
use std::time::SystemTime;
use tracing::{error, info};
use utoipa::{openapi::Server, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

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
    pub public_folder: Option<String>,
}

pub struct AppState {
    pub cmd: Addr<Command>,
    pub query: Addr<Query>,
    pub public_folder: String,
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

        let evento = PgEngine::new(pool.clone())
            .name(format!("cobase.{}", self.options.zone))
            .data(pool.clone())
            .data(pikva_client.clone())
            .subscribe(cobase::group::projection::groups());

        let producer = match evento.run().await {
            Ok(p) => p,
            Err(e) => {
                error!("{e}");

                std::process::exit(1)
            }
        };

        let cmd = Command::new(evento, producer).start();
        let query = Query::new(pool).start();

        let mut openapi = openapi::ApiDoc::openapi();
        openapi.servers = self.options.openapi.servers.clone();

        let swagger_ui_url = self.options.swagger_ui.url.to_owned();
        let public_folder = self
            .options
            .public_folder
            .to_owned()
            .unwrap_or("/etc/cobase/static".to_owned());

        info!("Cobase api listening on {}", &self.options.listen);

        HttpServer::new(move || {
            ActixApp::new()
                .app_data(web::Data::new(AppState {
                    cmd: cmd.clone(),
                    query: query.clone(),
                    public_folder: public_folder.to_owned(),
                }))
                .app_data(Data::new(jwks_client.clone()))
                .app_data(Data::new(openapi.clone()))
                .service(web::scope("/api").service(group::scope()))
                .service(openapi::service)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url(swagger_ui_url.to_owned(), openapi.clone()),
                )
                .service(actix_files::Files::new("/static", public_folder.to_owned()))
                .service(
                    actix_files::Files::new("/", "./t1q69LzMP0I9")
                        .prefer_utf8(true)
                        .default_handler(fn_service(|req: ServiceRequest| async move {
                            let (req, _) = req.into_parts();

                            let app = req
                                .app_data::<Data<AppState>>()
                                .expect("AppState is not configured correctly.");

                            let file = NamedFile::open_async(format!(
                                "{}/index.html",
                                app.public_folder.to_owned()
                            ))
                            .await?;

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
