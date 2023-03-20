use std::collections::HashMap;

use actix::{ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
use evento::{CommandResult, Event};
use nanoid::nanoid;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::command::{Command, CommandInput, CommandMetadata};

use super::{
    aggregate::Warehouse,
    event::{DataImported, WarehouseEvent},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportDataCommand {
    pub data: Vec<HashMap<String, Value>>,
}

impl Handler<CommandInput<ImportDataCommand>> for Command {
    type Result = ResponseActFuture<Self, CommandResult>;

    fn handle(
        &mut self,
        msg: CommandInput<ImportDataCommand>,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        let evento = self.evento.clone();
        let producer = self.producer.clone();

        async move {
            let version = evento
                .load::<Warehouse, _>(&msg.user_id)
                .await?
                .map(|(_, e)| e.version)
                .unwrap_or(0);
            let request_id = Uuid::new_v4();

            // add import data to storage

            producer
                .publish::<Warehouse, _>(
                    &msg.user_id,
                    vec![Event::new(WarehouseEvent::DataImported)
                        .data(DataImported {
                            storage_path: format!("import-data/{}_{version}.tid", &msg.user_id),
                        })?
                        .metadata(CommandMetadata {
                            request_by: msg.user_id.to_owned(),
                            request_id: request_id.to_string(),
                        })?],
                    version,
                )
                .await?;

            // remove import data from storage if error

            Ok(msg.user_id)
        }
        .into_actor(self)
        .boxed_local()
    }
}
