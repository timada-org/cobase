use evento::Aggregate;
use serde::{Deserialize, Serialize};

use super::event::{Created, RoomEvent};

#[derive(Default, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
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
