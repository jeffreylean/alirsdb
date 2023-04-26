use tonic::{transport::Server, Request, Response, Status};

// include_proto can only be used when the output is not modified.
// https://docs.rs/tonic/latest/tonic/macro.include_proto.html
mod ingest {
    tonic::include_proto!("alirs.ingest");
}
mod types {
    tonic::include_proto!("alirs.types");
}

#[derive(Default)]
pub struct AlirsServer {}

#[tonic::async_trait]
impl ingest::ingest_service_server::IngestService for AlirsServer {
    async fn ingest(
        &self,
        _request: Request<ingest::IngestRequest>,
    ) -> std::result::Result<Response<ingest::IngestResponse>, Status> {
        Ok(Response::new(ingest::IngestResponse {}))
    }
}

pub async fn start(port: u32) -> Result<(), tonic::transport::Error> {
    let server = AlirsServer::default();
    let addr = format!("0.0.0.0:{port:}").parse().unwrap();
    Server::builder()
        .add_service(ingest::ingest_service_server::IngestServiceServer::new(
            server,
        ))
        .serve(addr)
        .await?;
    Ok(())
}
