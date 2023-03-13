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
    pub data: Vec<Value>,
}

impl Handler<CommandInput<ImportDataCommand>> for Command {
    type Result = ResponseActFuture<Self, CommandResult>;

    fn handle(
        &mut self,
        msg: CommandInput<ImportDataCommand>,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        todo!()
        // let producer = self.producer.clone();

        // async move {
        //     let request_id = Uuid::new_v4();

        //     producer
        //         .publish::<Warehouse, _>(
        //             &id,
        //             vec![Event::new(WarehouseEvent::ContactsImported)
        //                 .data(ContactsImported {
        //                     storage_path: ,
        //                 })?
        //                 .metadata(CommandMetadata {
        //                     request_by: msg.user_id,
        //                     request_id: request_id.to_string(),
        //                 })?],
        //             0,
        //         )
        //         .await?;

        //     Ok(id)
        // }
        // .into_actor(self)
        // .boxed_local()
    }
}
