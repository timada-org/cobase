use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum WarehouseEvent {
    DataImported,
}

impl From<WarehouseEvent> for String {
    fn from(o: WarehouseEvent) -> Self {
        o.to_string()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct DataImported {
    pub storage_path: String,
}
