use actix::{ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
use evento::{CommandInfo, CommandResult, Event};
use nanoid::nanoid;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::command::{Command, CommandInput, CommandMetadata};

use super::event::{Created, GroupEvent};

#[derive(Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommand {
    pub name: String,
}

impl Handler<CommandInput<CreateCommand>> for Command {
    type Result = ResponseActFuture<Self, CommandResult>;

    fn handle(
        &mut self,
        msg: CommandInput<CreateCommand>,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        async move {
            let aggregate_id = nanoid!();
            let request_id = Uuid::new_v4();

            Ok(CommandInfo {
                aggregate_id,
                original_version: 0,
                events: vec![Event::new(GroupEvent::Created)
                    .data(Created {
                        name: msg.input.name,
                    })?
                    .metadata(CommandMetadata {
                        user_id: msg.user_id,
                        request_id: request_id.to_string(),
                    })?],
            })
        }
        .into_actor(self)
        .boxed_local()
    }
}
