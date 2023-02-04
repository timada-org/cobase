use evento::Aggregate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::event::{Created, GroupEvent};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct Group {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,

    #[schema(example = "My group name 1")]
    pub name: String,

    #[schema(example = "a18aac51-6262-4576-8883-7fda0ca72aac")]
    pub user_id: Uuid,
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
