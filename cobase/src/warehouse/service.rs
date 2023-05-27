use opendal::{Error, Operator, Result};
use serde_json::Value;
use std::collections::HashMap;

pub fn get_import_data_path() -> String {
    let id = nanoid::nanoid!();
    format!("import-data/{id}.tid")
}

pub async fn read_import_data(op: &Operator, path: &str) -> Result<Vec<HashMap<String, Value>>> {
    let content = op.read(path).await?;

    serde_json::from_slice(&content)
        .map_err(|e| Error::new(opendal::ErrorKind::Unexpected, &e.to_string()))
}

pub async fn write_import_data(
    op: &Operator,
    path: &str,
    data: Vec<HashMap<String, Value>>,
) -> Result<()> {
    let content = serde_json::to_string(&data)
        .map_err(|e| Error::new(opendal::ErrorKind::Unexpected, &e.to_string()))?;

    op.write(path, content).await
}
