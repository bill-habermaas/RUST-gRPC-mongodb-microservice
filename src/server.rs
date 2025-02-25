use tonic::{transport::Server, Request, Response, Status};

use dbase::dbase_server::{Dbase, DbaseServer};
//use auth::{LoginResponse, LoginRequest};

// Import the generated proto-rust file into a module
pub mod dbase {
    tonic::include_proto!("dbase");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyDbase {}

// Implement the service function(s) defined in the proto
#[tonic::async_trait]
impl Dbase for MyDbase {
    async fn connect(
        &self,
        request: Request<dbase::ConnectRequest>,
    ) -> Result<Response<dbase::ConnectResponse>, Status> {
        println!("Received request from: {:?}", request);

        let response = dbase::ConnectResponse {
            token: format!("Hello {}!", request.into_inner().username).into(),
            message: "howdy".to_string(),
        };

        Ok(Response::new(response))
    }
}

// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let serv = MyDbase::default();

    println!("Starting gRPC Mongodb server...");
    Server::builder()
        .add_service(DbaseServer::new(serv))
        .serve(addr)
        .await?;

    Ok(())
}
