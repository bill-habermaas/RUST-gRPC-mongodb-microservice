
use tonic::{transport::Server, Request, Response, Status};
extern crate mongodb;
use mongodb::Database;
use mongodb::{Client, options::{ClientOptions}};
use dbase::dbase_server::{Dbase, DbaseServer};
use crate::dbase::{DbaseStatus};

mod motd;
mod util;
mod users;

//Todo Add setting of MOTD
//Todo Finish getMOTD
//Todo code for user update
//Todo generate ObjectId used for all operations
//Todo Add time stamps to user and motd records
//Todo Add apache license headers to each source module

use once_cell::sync::OnceCell;

static MONGODB: OnceCell<Database> = OnceCell::new();

pub async fn initialize(dbspec: String, dbname: String) {
    if MONGODB.get().is_some() {
        return;
    }
    if let Ok(client_options) = ClientOptions::parse(dbspec.as_str()).await {
        if let Ok(client) = Client::with_options(client_options) {
            let _ = MONGODB.set(client.database(dbname.as_str()));
        }
    }
}

// Initialize and start the database
async fn start_database (dbspec: &String, dbname: &String) -> DbaseStatus {
    if MONGODB.get().is_some() == false {
        if let Ok(client_options) = ClientOptions::parse(dbspec.as_str()).await {
            if let Ok(client) = Client::with_options(client_options) {
                let _ = MONGODB.set(client.database(dbname));
            }
        }
    }
    let rsp = dbase::DbaseStatus {
        success: true,
        error_message: "".to_string(),
    };
    rsp
}

// Import the generated proto-rust file into a module
pub mod dbase {
    tonic::include_proto!("dbase");
}

// Implement the service skeleton for the service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyDbase {
}

// Implement the service function(s) defined in the proto
#[tonic::async_trait]
impl Dbase for MyDbase {

    async fn dbinit(&self,
        request: Request<dbase::DbinitRequest>,) -> Result<Response<dbase::DbaseStatus>, Status> {
        let req = request.into_inner();
        let response= start_database(&req.dbspec, &req.dbname).await;
        Ok(Response::new(response))
    }

    async fn getmotd(&self, request: Request<dbase::GetMotdRequest>,
        ) -> Result<Response<dbase::GetMotdResponse>, Status> {
        let req = request.into_inner();
        let response = motd::handle_getmotd(req.motd_filter).await;
        Ok(Response::new(response))
    }

    async fn getuser(&self,
        request: Request<dbase::GetUserRequest>,) -> Result<Response<dbase::GetUserResponse>, Status> {
        let req = request.into_inner();
        let response = users::handle_getuser(&req.username).await;
        Ok(Response::new(response))
    }

    async fn setuser(&self,
        request: Request<dbase::SetUserRequest>,) -> Result<Response<dbase::SetUserResponse>, Status> {
        let req = request.into_inner();
        let response = users::handle_setuser(&req).await;
        Ok(Response::new(response))
    }

    async fn deluser(&self, request: Request<dbase::DelUserRequest>,) -> Result<Response<dbase::DbaseStatus>, Status> {
        let req = request.into_inner();
        let response = users::handle_deluser(&req.username).await;
        Ok(Response::new(response))
    }

    async fn chkuser(&self, request: Request<dbase::CheckUserRequest>,) -> Result<Response<dbase::CheckUserResponse>, Status> {
        let req = request.into_inner();
        let response = users::handle_chkuser(&req.username).await;
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