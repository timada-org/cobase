use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum GroupEvent {
    Created,
}

impl From<GroupEvent> for String {
    fn from(o: GroupEvent) -> Self {
        o.to_string()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Created {
    pub name: String,
}
