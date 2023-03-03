use actix::{Actor, Context};
use sqlx::PgPool;

pub struct Query {
    pub pool: PgPool,
}

impl Query {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Actor for Query {
    type Context = Context<Self>;
}
