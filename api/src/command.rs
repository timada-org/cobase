use actix::{Actor, Context, Message};
use actix_jwks::JwtPayload;
use evento::{CommandResult, Evento, PgEngine};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata {
    pub user_id: String,
    pub request_id: String,
}

pub struct Command {
    pub store: Evento<PgEngine, evento::store::PgEngine>,
}

impl Command {
    pub fn new(store: Evento<PgEngine, evento::store::PgEngine>) -> Self {
        Self { store }
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
