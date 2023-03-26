use opendal::{layers::LoggingLayer, services, Operator, Result};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct FsStorage {
    pub root: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Storage {
    pub fs: Option<FsStorage>,
}

impl Storage {
    pub fn build(&self) -> Result<Operator> {
        if let Some(fs) = &self.fs {
            let mut builder = services::Fs::default();
            builder.root(&fs.root);

            return Ok(Operator::new(builder)?
                .layer(LoggingLayer::default())
                .finish());
        }

        Ok(Operator::new(services::Memory::default())?
            .layer(LoggingLayer::default())
            .finish())
    }
}
