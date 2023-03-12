use evento::Aggregate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::event::{Created, RoomEvent};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Room {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,

    #[schema(example = "My room name 1")]
    pub name: String,

    #[schema(example = "a18aac51-6262-4576-8883-7fda0ca72aac")]
    pub user_id: Uuid,
}

impl Aggregate for Room {
    fn apply(&mut self, event: &evento::Event) {
        let room_event: RoomEvent = event.name.parse().unwrap();

        match room_event {
            RoomEvent::Created => {
                let data: Created = event.to_data().unwrap();
                self.name = data.name;
            }
        }
    }

    fn aggregate_type<'a>() -> &'a str {
        "room"
    }
}
