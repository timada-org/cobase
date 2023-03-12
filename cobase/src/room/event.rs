use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum RoomEvent {
    Created,
}

impl From<RoomEvent> for String {
    fn from(o: RoomEvent) -> Self {
        o.to_string()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Created {
    pub name: String,
}
