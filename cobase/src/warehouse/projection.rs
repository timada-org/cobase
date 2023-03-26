use chrono::{DateTime, Utc};
use evento::{query::Cursor, SubscirberHandlerError, Subscriber};
use futures::FutureExt;
use nanoid::nanoid;
use opendal::Operator;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{command::CommandMetadata, warehouse::event::WarehouseEvent};

use super::{event::DataImported, service::read_import_data};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, FromRow)]
pub struct Warehouse {
    pub id: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}
#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, FromRow)]
pub struct WarehouseData {
    pub id: String,
    pub key: String,
    pub data: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Cursor for WarehouseData {
    fn keys() -> Vec<&'static str> {
        vec!["created_at", "key"]
    }

    fn bind<'q, O>(
        self,
        query: sqlx::query::QueryAs<Postgres, O, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<Postgres, O, sqlx::postgres::PgArguments>
    where
        O: for<'r> FromRow<'r, <sqlx::Postgres as sqlx::Database>::Row>,
        O: 'q + std::marker::Send,
        O: 'q + Unpin,
        O: 'q + Cursor,
    {
        query.bind(self.created_at).bind(self.key)
    }

    fn serialize(&self) -> Vec<String> {
        vec![Self::serialize_utc(self.created_at), self.key.to_owned()]
    }

    fn deserialize(values: Vec<&str>) -> Result<Self, evento::query::CursorError> {
        let mut values = values.iter();
        let created_at = Self::deserialize_as_utc("created_at", values.next())?;
        let key = Self::deserialize_as("key", values.next())?;

        Ok(WarehouseData {
            key,
            created_at,
            ..Default::default()
        })
    }
}

pub fn warehouse_data() -> Subscriber {
    Subscriber::new("warehouse-data")
        .filter("warehouse/#")
        .handler(|event, ctx| {
            let db = ctx.0.read().extract::<PgPool>().clone();
            let pikav = ctx.0.read().extract::<pikav_client::Client>().clone();
            let op = ctx.0.read().extract::<Operator>().clone();

            async move {
                let warehouse_event: WarehouseEvent = event.name.parse()?;
                let metadata = event.to_metadata::<CommandMetadata>()?;

                match warehouse_event {
                    WarehouseEvent::DataImported => {
                        let data: DataImported = event.to_data()?;

                        let import_data =
                            read_import_data(&op, &data.storage_path)
                                .await
                                .map_err(|e| SubscirberHandlerError::new("warehouse-data.read_import_data", e.to_string()))?;


                        let warehouse_id = sqlx::query_as::<_, (String,)>(
                            "SELECT id FROM warehouses WHERE user_id = $1",
                        )
                        .bind(&metadata.request_by)
                        .fetch_optional(&db)
                        .await?;

                        let warehouse_id = match warehouse_id {
                            Some((id,)) => id,
                            _ => {
                                let id = nanoid!().replace('-', "").replace('_', "");
                                let mut tx = db.begin().await?;

                                let res = sqlx::query::<_>(
                                    "INSERT INTO warehouses (id, user_id, created_at) VALUES ($1, $2, $3)",
                                )
                                .bind(&id)
                                .bind(&metadata.request_by)
                                .bind(event.created_at)
                                .execute(&mut *tx)
                                .await;

                                if let Err(e) = res {
                                    tx.rollback().await?;
                                    return Err(e.into());
                                }

                                let res = sqlx::query::<_>(
                                    &format!(r#"
                                    CREATE TABLE warehouse_data_{id}
                                    (
                                        id VARCHAR(21) NOT NULL PRIMARY KEY,
                                        key VARCHAR(50) NOT NULL,
                                        data json NOT NULL,
                                        created_at timestamptz NOT NULL,
                                        updated_at timestamptz NULL
                                    )
                                    "#),
                                )
                                .execute(&mut *tx)
                                .await;

                                if let Err(e) = res {
                                    tx.rollback().await?;
                                    return Err(e.into());
                                }

                                let res = sqlx::query::<_>(
                                    &format!("CREATE UNIQUE INDEX ON warehouse_data_{id} (key)"),
                                )
                                .execute(&mut *tx)
                                .await;

                                if let Err(e) = res {
                                    tx.rollback().await?;
                                    return Err(e.into());
                                }

                                tx.commit().await?;

                                id
                            }
                        };

                        for import_data in import_data.chunks(1000).collect::<Vec<&[_]>>() {
                            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                                format!("INSERT INTO warehouse_data_{warehouse_id} (id, key, data, created_at) ")
                            );

                            let mut errors = Vec::new();
                            let mut data_keys = Vec::new();

                            query_builder.push_values(import_data, |mut b, data| {
                                let key = match data.get("_id").cloned() {
                                    Some(key) => match key {
                                        Value::Number(v) => Some(v.to_string()),
                                        Value::String(v) => Some(v),
                                        _ => None,
                                    },
                                    _ => {
                                        None
                                    },
                                };

                                let key = match key {
                                    Some(key) => key,
                                    None => {
                                        errors.push(SubscirberHandlerError::new("warehouse-data.query_builder.push_values.id", "missing field _id"));
                                        return
                                    },
                                };

                                let data = match serde_json::to_value(data) {
                                    Ok(data) => data,
                                    Err(e) => {
                                        errors.push(SubscirberHandlerError::new("warehouse-data.query_builder.push_values.serde", e.to_string()));
                                        return
                                    },
                                };

                                data_keys.push(key.to_owned());

                                b.push_bind(nanoid!())
                                .push_bind(key)
                                .push_bind(data)
                                .push_bind(event.created_at);
                            });

                            if let Some(e) = errors.first() {
                                return Err(e.clone())
                            }

                            query_builder.push(r#"
                                ON CONFLICT (key)
                                DO UPDATE SET data = EXCLUDED.data, updated_at = EXCLUDED.created_at
                            "#);

                            query_builder.build().execute(&db).await?;

                            let warehouse_data = sqlx::query_as::<_, WarehouseData>(
                                &format!("SELECT * FROM warehouse_data_{warehouse_id} WHERE key = ANY($1)"),
                            )
                            .bind(&data_keys[..])
                            .fetch_all(&db)
                            .await?;

                            pikav.publish(vec![pikav_client::Event {
                                user_id: metadata.request_by.to_owned(),
                                topic: format!("warehouses/{}", warehouse_id),
                                name: "data-imported".to_owned(),
                                data: Some(serde_json::to_value(warehouse_data)?.into()),
                                metadata: None,
                            }]);
                        }
                    }
                };

                Ok(())
            }
            .boxed()
        })
}
