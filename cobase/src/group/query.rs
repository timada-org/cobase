use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use evento::CommandError;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::query::Query;

use super::projection::Group;

#[derive(Message, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "Result<Vec<Group>, CommandError>")]
pub struct ListGroupsQuery {
    pub user_id: Uuid,
}

impl Handler<ListGroupsQuery> for Query {
    type Result = ResponseActFuture<Self, Result<Vec<Group>, CommandError>>;

    fn handle(&mut self, msg: ListGroupsQuery, _ctx: &mut Context<Self>) -> Self::Result {
        let pool = self.pool.clone();

        async move {
            let groups = sqlx::query_as!(
                Group,
                "SELECT * FROM groups WHERE user_id = $1",
                &msg.user_id
            )
            .fetch_all(&pool)
            .await?;

            Ok(groups)
        }
        .into_actor(self)
        .boxed_local()
    }
}
