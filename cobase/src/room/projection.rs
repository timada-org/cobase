use chrono::{DateTime, Utc};
use evento::{Aggregate, Subscriber};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{command::CommandMetadata, room::event::RoomEvent};

use super::{
    aggregate::{self},
    event::Created,
};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, FromRow)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub fn rooms() -> Subscriber {
    Subscriber::new("rooms")
        .filter("room/#")
        .handler(|event, ctx| {
            let db = ctx.0.read().extract::<PgPool>().clone();
            let pikav = ctx.0.read().extract::<pikav_client::Client>().clone();

            async move {
                let room_event: RoomEvent = event.name.parse()?;
                let metadata = event.to_metadata::<CommandMetadata>()?;

                match room_event {
                    RoomEvent::Created => {
                        let data: Created = event.to_data()?;

                        let room = Room {
                            id: aggregate::Room::to_id(event.aggregate_id),
                            name: data.name,
                            user_id: Uuid::parse_str(&metadata.request_by)?,
                            created_at: event.created_at,
                        };

                        sqlx::query::<_>(
                            "INSERT INTO rooms (id, name, user_id, created_at) VALUES ($1, $2, $3, $4)",
                        )
                        .bind(&room.id)
                        .bind(&room.name)
                        .bind(room.user_id)
                        .bind(room.created_at)
                        .execute(&db)
                        .await?;

                        pikav.publish(vec![pikav_client::Event {
                            user_id: metadata.request_by,
                            topic: format!("rooms/{}", room.id),
                            name: "created".to_owned(),
                            data: Some(serde_json::to_value(room)?.into()),
                            metadata: None,
                        }]);
                    }
                };

                Ok(())
            }
            .boxed()
        })
}
