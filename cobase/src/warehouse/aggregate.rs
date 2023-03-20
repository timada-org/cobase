use evento::Aggregate;
use serde::{Deserialize, Serialize};

use super::event::{DataImported, WarehouseEvent};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Warehouse {
    pub storage_paths: Vec<String>,
}

impl Aggregate for Warehouse {
    fn apply(&mut self, event: &evento::Event) {
        let warehouse_event: WarehouseEvent = event.name.parse().unwrap();

        match warehouse_event {
            WarehouseEvent::DataImported => {
                let data: DataImported = event.to_data().unwrap();
                self.storage_paths.push(data.storage_path);
            }
        }
    }

    fn aggregate_type<'a>() -> &'a str {
        "warehouse"
    }
}
