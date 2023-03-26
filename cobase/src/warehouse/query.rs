use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use evento::{
    query::{Query as QueryAs, QueryArgs, QueryResult},
    CommandError,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::query::Query;

use super::projection::WarehouseData;

#[derive(Message, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "Result<QueryResult<WarehouseData>, CommandError>")]
pub struct ListWarehouseDataQuery {
    pub user_id: Uuid,
    pub query_args: QueryArgs,
}

impl Handler<ListWarehouseDataQuery> for Query {
    type Result = ResponseActFuture<Self, Result<QueryResult<WarehouseData>, CommandError>>;

    fn handle(&mut self, msg: ListWarehouseDataQuery, _ctx: &mut Context<Self>) -> Self::Result {
        let db = self.pool.clone();

        async move {
            let warehouse_id =
                sqlx::query_as::<_, (String,)>("SELECT id FROM warehouses WHERE user_id = $1")
                    .bind(&msg.user_id.to_string())
                    .fetch_optional(&db)
                    .await?;

            let warehouse_id = match warehouse_id {
                Some((warehouse_id,)) => warehouse_id,
                None => return Ok(QueryResult::default()),
            };

            let res = QueryAs::<WarehouseData>::new(&format!(
                "SELECT * FROM warehouse_data_{warehouse_id}"
            ))
            .build(msg.query_args)
            .fetch_all(&db)
            .await?;

            Ok(res)
        }
        .into_actor(self)
        .boxed_local()
    }
}
