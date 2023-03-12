use error::ClientError;
use serde::Deserialize;
use timada::{cobase_client::CobaseClient, CreateRoomReply};
use tonic::transport::Channel;

pub use timada::CreateRoomRequest;
pub use tonic::Status;

mod error;

pub mod timada {
    tonic::include_proto!("timada");
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClientOptions<N: Into<String>> {
    pub url: String,
    pub namespace: N,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClientInstanceOptions {
    pub url: String,
    pub namespace: Option<String>,
}

#[derive(Clone)]
pub struct Client {
    channel: Channel,
}

impl Client {
    pub fn new<N: Into<String>>(url: N) -> Result<Self, ClientError> {
        let channel = Channel::from_shared(url.into())
            .map_err(|e| ClientError::Unknown(e.to_string()))?
            .connect_lazy();

        Ok(Self { channel })
    }

    pub async fn create_room(
        &self,
        message: CreateRoomRequest,
    ) -> Result<tonic::Response<CreateRoomReply>, Status> {
        let mut client = CobaseClient::new(self.channel.clone());

        let request = tonic::Request::new(message);

        client.create_room(request).await
    }
}
