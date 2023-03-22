use std::collections::HashMap;

use opendal::{Error, Operator, Result};
use serde_json::Value;

pub(crate) fn get_import_data_path(user_id: &str, version: i32) -> String {
    format!("import-data/{}_{version}.tid", &user_id)
}

pub(crate) async fn read_import_data(
    op: &Operator,
    user_id: &str,
    version: i32,
) -> Result<Vec<HashMap<String, Value>>> {
    let path = get_import_data_path(user_id, version);
    let content = op.read(&path).await?;

    Ok(serde_json::from_slice(&content)
        .map_err(|e| Error::new(opendal::ErrorKind::Unexpected, &e.to_string()))?)
}

pub(crate) async fn write_import_data(
    op: &Operator,
    user_id: &str,
    version: i32,
    data: Vec<HashMap<String, Value>>,
) -> Result<()> {
    let path = get_import_data_path(user_id, version);
    let conent = serde_json::to_string(&data)
        .map_err(|e| Error::new(opendal::ErrorKind::Unexpected, &e.to_string()))?;

    op.write(&path, conent).await
}
