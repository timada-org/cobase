use actix::prelude::*;
use actix_jwks::JwtPayload;
use actix_web::HttpResponse;
use evento::{Aggregate, Engine, Event, EventStore, PgEngine};
use pulsar::{producer, DeserializeMessage, Payload, Producer, SerializeMessage, TokioExecutor};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandInfo {
    pub aggregate_id: String,
    pub original_version: i32,
    pub events: Vec<Event>,
}

impl From<Event> for CommandInfo {
    fn from(e: Event) -> Self {
        Self {
            aggregate_id: e.aggregate_id.to_owned(),
            original_version: e.version,
            events: vec![e],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMessage {
    pub event: Event,
}

impl SerializeMessage for CommandMessage {
    fn serialize_message(input: Self) -> Result<producer::Message, pulsar::Error> {
        let payload =
            serde_json::to_vec(&input).map_err(|e| pulsar::Error::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}

impl DeserializeMessage for CommandMessage {
    type Output = Result<CommandMessage, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

pub type CommandResult = Result<CommandInfo, Error>;

pub struct Command {
    pub store: EventStore<PgEngine>,
}

impl Command {
    pub fn new(pool: PgPool) -> Self {
        Self {
            store: PgEngine::new(pool),
        }
    }
}

impl Actor for Command {
    type Context = Context<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata {
    pub user_id: String,
    pub request_id: String,
    pub zone: String,
}

pub struct CommandResponse(pub Result<CommandResult, MailboxError>);

impl CommandResponse {
    pub async fn to_response<A: Aggregate>(
        self,
        zone: &str,
        store: &EventStore<PgEngine>,
        producer: &mut Producer<TokioExecutor>,
        payload: JwtPayload,
    ) -> Result<HttpResponse, Error> {
        let info = self.0??;
        let mut events = Vec::new();
        let request_id = Uuid::new_v4().to_string();

        for event in info.events.into_iter() {
            events.push(event.metadata(CommandMetadata {
                user_id: payload.subject.to_owned(),
                zone: zone.to_owned(),
                request_id: request_id.to_owned(),
            })?);
        }

        let events = store
            .save::<A, _>(&info.aggregate_id, events, info.original_version)
            .await
            .map(|events| {
                events
                    .into_iter()
                    .map(|e| CommandMessage { event: e })
                    .collect::<Vec<CommandMessage>>()
            })?;

        producer.send_all(events).await?;

        Ok(HttpResponse::Ok().json(json!({
            "id": info.aggregate_id
        })))
    }
}
