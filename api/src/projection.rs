use evento::Event;
use futures::{Future, TryStreamExt};
use pulsar::{Consumer, Pulsar, SubType, TokioExecutor};
use sqlx::PgPool;
use tracing::error;

use crate::{
    command::{CommandMessage, CommandMetadata},
    error::Error,
    AppOptions,
};

pub struct Projection<'a> {
    pub pulsar: &'a Pulsar<TokioExecutor>,
    pub db: &'a PgPool,
    pub pikav: &'a pikav_client::Client,
    pub options: &'a AppOptions,
}

impl<'a> Projection<'a> {
    pub async fn spawn<T>(
        &self,
        name: &'a str,
        handler: fn(pikav_client::Client, PgPool, Event, CommandMetadata) -> T,
    ) -> Result<(), pulsar::Error>
    where
        T: Future<Output = Result<(), Error>> + Send + 'static,
    {
        let consumer_name = format!("cobase.{}", self.options.zone);
        let mut consumer: Consumer<CommandMessage, _> = self
            .pulsar
            .consumer()
            .with_topic(format!("{}/{}", self.options.pulsar.namespace, name))
            .with_consumer_name(&consumer_name)
            .with_subscription_type(SubType::Exclusive)
            .with_subscription(&consumer_name)
            .build()
            .await?;

        let db = self.db.clone();
        let pikav = self.pikav.clone();

        tokio::spawn(async move {
            while let Ok(msg) = consumer.try_next().await {
                let msg = match msg {
                    Some(msg) => msg,
                    None => continue,
                };

                if let Err(e) = consumer.ack(&msg).await {
                    error!("{e}");
                    break;
                }

                let event = match msg.deserialize() {
                    Ok(msg) => msg.event,
                    Err(e) => {
                        error!("{e}");
                        continue;
                    }
                };

                let metadata = match event.to_metadata::<CommandMetadata>() {
                    Ok(metadata) => match metadata {
                        Some(metadata) => metadata,
                        _ => {
                            error!("metadata not defined for `{}`", event.id);
                            continue;
                        }
                    },
                    Err(e) => {
                        error!("{e}");
                        continue;
                    }
                };

                if let Err(Error::InternalServerErr(e)) =
                    handler(pikav.clone(), db.clone(), event, metadata).await
                {
                    error!("{e}");
                }
            }
        });

        Ok(())
    }
}
