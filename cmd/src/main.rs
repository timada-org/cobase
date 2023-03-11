mod migrate;
mod openapi;
mod reset;
mod serve;

use clap::{arg, Command};
use futures::{Future, TryFutureExt};
use migrate::Migrate;
use openapi::OpenApiCmd;
use reset::Reset;
use serve::Serve;
use std::{io, time::Duration};

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
            Command::new("reset")
                .about("reset codebase database")
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
        Some(("reset", sub_matches)) => {
            let s = match Reset::new(
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

/// Attempt to connect to the database server, retrying up to `ops.connect_timeout`.
// async fn connect(database_url: &String) -> sqlx::Result<AnyConnection> {
//     retry_connect_errors(database_url, AnyConnection::connect).await
// }

/// Attempt an operation that may return errors like `ConnectionRefused`,
/// retrying up until `ops.connect_timeout`.
///
/// The closure is passed `&ops.database_url` for easy composition.
async fn retry_connect_errors<'a, F, Fut, T>(
    database_url: &'a str,
    mut connect: F,
) -> sqlx::Result<T>
where
    F: FnMut(&'a str) -> Fut,
    Fut: Future<Output = sqlx::Result<T>> + 'a,
{
    // sqlx::any::install_default_drivers();

    backoff::future::retry(
        backoff::ExponentialBackoffBuilder::new()
            .with_max_elapsed_time(Some(Duration::from_secs(10)))
            .build(),
        || {
            connect(database_url).map_err(|e| -> backoff::Error<sqlx::Error> {
                if let sqlx::Error::Io(ref ioe) = e {
                    match ioe.kind() {
                        io::ErrorKind::ConnectionRefused
                        | io::ErrorKind::ConnectionReset
                        | io::ErrorKind::ConnectionAborted => {
                            return backoff::Error::transient(e);
                        }
                        _ => (),
                    }
                }

                backoff::Error::permanent(e)
            })
        },
    )
    .await
}
