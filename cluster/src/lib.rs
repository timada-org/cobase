use timada_cobase_client::timada::{
    cobase_server::CobaseServer, CreateGroupReply, CreateGroupRequest,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct Cobase {}

#[tonic::async_trait]
impl timada_cobase_client::timada::cobase_server::Cobase for Cobase {
    async fn create_group(
        &self,
        request: Request<CreateGroupRequest>,
    ) -> Result<Response<CreateGroupReply>, Status> {
        println!("Request create_group from {:?}", request.remote_addr());

        Ok(Response::new(CreateGroupReply { success: true }))
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

        println!("CobaseServer listening on {addr}");

        Server::builder()
            .add_service(CobaseServer::new(cobase))
            .serve(addr)
            .await
    }
}
