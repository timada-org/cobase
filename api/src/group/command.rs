use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use evento::{Event, CommandInfo, CommandResult};
use nanoid::nanoid;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::command::{Command};

use super::event::{Created, GroupEvent};

#[derive(Message, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "CommandResult")]
pub struct CreateCommand {
    pub name: String,
}

impl Handler<CreateCommand> for Command {
    type Result = ResponseActFuture<Self, CommandResult>;

    fn handle(&mut self, msg: CreateCommand, _ctx: &mut Context<Self>) -> Self::Result {
        async move {
            let aggregate_id = nanoid!();

            Ok(CommandInfo {
                aggregate_id,
                original_version: 0,
                events: vec![Event::new(GroupEvent::Created).data(Created { name: msg.name })?],
            })
        }
        .into_actor(self)
        .boxed_local()
    }
}
