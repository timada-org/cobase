use actix::{Actor, Context, Message};
use actix_jwks::JwtPayload;
use evento::{CommandResult, PgEvento, PgProducer};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata {
    pub user_id: String,
    pub request_id: String,
}

pub struct Command {
    pub evento: PgEvento,
    pub producer: PgProducer,
}

impl Command {
    pub fn new(evento: PgEvento, producer: PgProducer) -> Self {
        Self { evento, producer }
    }
}

impl Actor for Command {
    type Context = Context<Self>;
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CommandJsonResponse {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,
}

#[derive(Message, Deserialize)]
#[rtype(result = "CommandResult")]
pub struct CommandInput<I> {
    pub user_id: String,
    pub input: I,
}

impl<I> CommandInput<I> {
    pub fn new(payload: JwtPayload, input: I) -> Self {
        Self {
            input,
            user_id: payload.subject,
        }
    }
}
