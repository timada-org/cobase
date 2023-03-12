use timada_cobase_client::timada::{
    cobase_server::CobaseServer, CreateRoomReply, CreateRoomRequest,
};
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;

#[derive(Default)]
pub struct Cobase {}

#[tonic::async_trait]
impl timada_cobase_client::timada::cobase_server::Cobase for Cobase {
    async fn create_room(
        &self,
        _request: Request<CreateRoomRequest>,
    ) -> Result<Response<CreateRoomReply>, Status> {
        Ok(Response::new(CreateRoomReply { success: true }))
    }
}

pub struct ClusterOptions {
    pub addr: String,
}

pub struct Cluster {
    pub options: ClusterOptions,
}

impl Cluster {
    pub fn new(options: ClusterOptions) -> Self {
        Self { options }
    }

    pub async fn serve(&self) -> Result<(), tonic::transport::Error> {
        let addr = self.options.addr.parse().unwrap();
        let cobase = Cobase {};

        info!("Cobase grpc listening on {addr}");

        Server::builder()
            .add_service(CobaseServer::new(cobase))
            .serve(addr)
            .await
    }
}
