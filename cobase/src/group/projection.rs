use evento::{Aggregate, Subscriber};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{command::CommandMetadata, group::event::GroupEvent};

use super::{
    aggregate::{self},
    event::Created,
};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub user_id: Uuid,
}

pub fn groups() -> Subscriber {
    Subscriber::new("groups")
        .filter("group/#")
        .handler(|event, ctx| {
            let db = ctx.0.read().extract::<PgPool>().clone();
            let pikav = ctx.0.read().extract::<pikav_client::Client>().clone();

            async move {
                let group_event: GroupEvent = event.name.parse()?;
                let metadata = event.to_metadata::<CommandMetadata>()?;

                match group_event {
                    GroupEvent::Created => {
                        let data: Created = event.to_data()?;

                        let group = Group {
                            id: aggregate::Group::to_id(event.aggregate_id),
                            name: data.name,
                            user_id: Uuid::parse_str(&metadata.user_id)?,
                        };

                        sqlx::query!(
                            "INSERT INTO groups (id, name, user_id) VALUES ($1, $2, $3)",
                            &group.id,
                            &group.name,
                            &group.user_id
                        )
                        .execute(&db)
                        .await?;

                        pikav.publish(vec![pikav_client::Event {
                            user_id: metadata.user_id,
                            topic: format!("groups/{}", group.id),
                            name: "created".to_owned(),
                            data: Some(serde_json::to_value(group).unwrap().into()),
                            metadata: None,
                        }]);
                    }
                };

                Ok(())
            }
            .boxed()
        })
}
