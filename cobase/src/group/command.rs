use actix::{ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
use evento::{CommandResult, Event};
use nanoid::nanoid;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::command::{Command, CommandInput, CommandMetadata};

use super::{
    event::{Created, GroupEvent},
    Group,
};

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
        let producer = self.producer.clone();

        async move {
            let id = nanoid!();
            let request_id = Uuid::new_v4();

            producer
                .publish::<Group, _>(
                    &id,
                    vec![Event::new(GroupEvent::Created)
                        .data(Created {
                            name: msg.input.name,
                        })?
                        .metadata(CommandMetadata {
                            user_id: msg.user_id,
                            request_id: request_id.to_string(),
                        })?],
                    0,
                )
                .await?;

            Ok(id)
        }
        .into_actor(self)
        .boxed_local()
    }
}
