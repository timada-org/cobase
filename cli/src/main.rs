mod migrate;
mod openapi;
mod serve;

use clap::{arg, Command};
use migrate::Migrate;
use openapi::OpenApiCmd;
use serve::Serve;

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("serve")
                .about("Run the cobase api")
                .arg(arg!(-c --config <CONFIG>).required(false)),
        )
        .subcommand(
            Command::new("migrate")
                .about("Migrate codebase database")
                .arg(arg!(-c --config <CONFIG>).required(false)),
        )
        .subcommand(
            Command::new("openapi")
                .about("Generate openapi doc")
                .arg(arg!(-c --config <CONFIG>).required(false)),
        )
}

#[actix_rt::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("serve", sub_matches)) => {
            let s = match Serve::new(
                sub_matches
                    .get_one::<String>("config")
                    .unwrap_or(&"".to_owned()),
            ) {
                Ok(s) => s,
                Err(e) => panic!("{e}"),
            };

            if let Err(e) = s.run().await {
                panic!("{e}");
            }
        }
        Some(("migrate", sub_matches)) => {
            let s = match Migrate::new(
                sub_matches
                    .get_one::<String>("config")
                    .unwrap_or(&"".to_owned()),
            ) {
                Ok(s) => s,
                Err(e) => panic!("{e}"),
            };

            if let Err(e) = s.run().await {
                panic!("{e}");
            }
        }
        Some(("openapi", sub_matches)) => {
            let s = match OpenApiCmd::new(
                sub_matches
                    .get_one::<String>("config")
                    .unwrap_or(&"".to_owned()),
            ) {
                Ok(s) => s,
                Err(e) => panic!("{e}"),
            };

            if let Err(e) = s.run().await {
                panic!("{e}");
            }
        }
        _ => unreachable!(),
    }
}
