use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use evento::CommandError;
use serde::Deserialize;
use uuid::Uuid;

use crate::query::Query;

use super::projection::Room;

#[derive(Message, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "Result<Vec<Room>, CommandError>")]
pub struct ListRoomsQuery {
    pub user_id: Uuid,
}

impl Handler<ListRoomsQuery> for Query {
    type Result = ResponseActFuture<Self, Result<Vec<Room>, CommandError>>;

    fn handle(&mut self, msg: ListRoomsQuery, _ctx: &mut Context<Self>) -> Self::Result {
        let pool = self.pool.clone();

        async move {
            let rooms =
                sqlx::query_as!(Room, "SELECT * FROM rooms WHERE user_id = $1", &msg.user_id)
                    .fetch_all(&pool)
                    .await?;

            Ok(rooms)
        }
        .into_actor(self)
        .boxed_local()
    }
}
