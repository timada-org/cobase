use evento::Aggregate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::event::{Created, GroupEvent};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Group {
    #[schema(example = "Transquadra")]
    pub name: String,
}

impl Aggregate for Group {
    fn apply(&mut self, event: &evento::Event) {
        let group_event: GroupEvent = event.name.parse().unwrap();

        match group_event {
            GroupEvent::Created => {
                let data: Created = event.to_data().unwrap();
                self.name = data.name;
            }
        }
    }

    fn aggregate_type<'a>() -> &'a str {
        "group"
    }
}
