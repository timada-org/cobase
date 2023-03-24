use std::collections::HashMap;

use actix::{ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
use evento::{CommandError, CommandResult, Event};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::command::{Command, CommandInput, CommandMetadata};

use super::{
    aggregate::Warehouse,
    event::{DataImported, WarehouseEvent},
    service::{get_import_data_path, write_import_data},
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
        let storage = self.storage.clone();

        async move {
            let error_pos = msg.input.data.iter().position(|o| match o.get("_id") {
                Some(id) => !matches!(id, Value::Number(_) | Value::String(_)),
                _ => true,
            });

            if let Some(pos) = error_pos {
                return Err(CommandError::BadRequest(format!(
                    "Missing field _id or not (string | number) at index {pos}"
                )));
            }

            let request_id = Uuid::new_v4();
            let version = evento
                .load::<Warehouse, _>(&msg.user_id)
                .await?
                .map(|(_, e)| e.version)
                .unwrap_or(0);

            let storage_path = get_import_data_path();
            let import_data_exists = storage
                .is_exist(&storage_path)
                .await
                .map_err(|e| CommandError::InternalServerErr(e.to_string()))?;

            if import_data_exists {
                return Err(CommandError::InternalServerErr(
                    "Import data kv already exist".to_owned(),
                ));
            }

            write_import_data(&storage, &storage_path, msg.input.data)
                .await
                .map_err(|e| CommandError::InternalServerErr(e.to_string()))?;

            let res = producer
                .publish::<Warehouse, _>(
                    &msg.user_id,
                    vec![Event::new(WarehouseEvent::DataImported)
                        .data(DataImported {
                            storage_path: storage_path.to_owned(),
                        })?
                        .metadata(CommandMetadata {
                            request_by: msg.user_id.to_owned(),
                            request_id: request_id.to_string(),
                        })?],
                    version,
                )
                .await;

            if res.is_err() {
                storage
                    .remove(vec![storage_path])
                    .await
                    .map_err(|e| CommandError::InternalServerErr(e.to_string()))?;
            }

            res?;

            Ok(msg.user_id)
        }
        .into_actor(self)
        .boxed_local()
    }
}
