use evento::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{group::event::GroupEvent, projection::Projection};

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

pub async fn start(projection: &Projection<'_>) -> Result<(), pulsar::Error> {
    projection
        .spawn("group", |pikav, db, event, metadata| async move {
            let group_event: GroupEvent = event.name.parse().unwrap();

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

                    pikav.publish(vec![pikav_client::Event::new(
                        metadata.user_id,
                        format!("groups/{}", group.id),
                        "created",
                        group,
                    )?]);
                }
            }

            Ok(())
        })
        .await?;

    Ok(())
}
