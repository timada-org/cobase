use actix::{Actor, Context};
use evento::{Evento, PgEngine};
use serde::{Deserialize, Serialize};

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
