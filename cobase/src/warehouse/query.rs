use actix::{ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use evento::CommandError;
use serde::Deserialize;
use uuid::Uuid;

use crate::query::Query;

use super::projection::WarehouseData;

#[derive(Message, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "Result<Vec<WarehouseData>, CommandError>")]
pub struct ListWarehouseDatasQuery {
    pub user_id: Uuid,
}

impl Handler<ListWarehouseDatasQuery> for Query {
    type Result = ResponseActFuture<Self, Result<Vec<WarehouseData>, CommandError>>;

    fn handle(&mut self, msg: ListWarehouseDatasQuery, _ctx: &mut Context<Self>) -> Self::Result {
        let db = self.pool.clone();

        async move {
            let warehouse_id =
                sqlx::query_as::<_, (String,)>("SELECT id FROM warehouses WHERE user_id = $1")
                    .bind(&msg.user_id.to_string())
                    .fetch_optional(&db)
                    .await?;

            let warehouse_id = match warehouse_id {
                Some((warehouse_id,)) => warehouse_id,
                None => return Ok(vec![]),
            };

            let warehouse_datas = sqlx::query_as::<_, WarehouseData>(&format!(
                "SELECT * FROM warehouse_datas_{warehouse_id} ORDER BY created_at, key"
            ))
            .fetch_all(&db)
            .await?;

            Ok(warehouse_datas)
        }
        .into_actor(self)
        .boxed_local()
    }
}
