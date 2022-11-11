use std::{io::Write, path::Path};

use api::ApiDoc;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use utoipa::OpenApi;

#[derive(Deserialize)]
pub struct OpenApiOptions {
    pub path: Option<String>,
}

#[derive(Deserialize)]
pub struct OpenApiCmd {
    pub openapi: Option<OpenApiOptions>,
}

impl OpenApiCmd {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path))
            .add_source(File::with_name(&format!("{}.local", path)).required(false))
            .add_source(Environment::with_prefix(env!("CARGO_PKG_NAME")))
            .build()?
            .try_deserialize()
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let openapi = ApiDoc::openapi()
            .to_json()
            .expect("failed to create api doc");
        let path = Path::new("../openapi.json");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(openapi.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        Ok(())
    }
}
