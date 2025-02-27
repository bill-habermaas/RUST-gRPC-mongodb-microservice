use tonic::{transport::Server, Request, Response, Status};
extern crate mongodb;
use mongodb::{Client, options::{ClientOptions}};
use dbase::dbase_server::{Dbase, DbaseServer};
use crate::dbase::DbaseStatus;
use mongodb::Database;

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

    async fn dbquit(&self,
        request: Request<dbase::DbquitRequest>,) -> Result<Response<dbase::DbaseStatus>, Status> {
        let req = request.into_inner();
        let response = stop_database(&req.reason).await;
        Ok(Response::new(response))
    }

    async fn getmotd(
        &self,
        request: Request<dbase::MotdRequest>,
    ) -> Result<Response<dbase::MotdResponse>, Status> {
        println!("Received request from: {:?}", request);
        let req = request.into_inner();
        let status = dbase::DbaseStatus {
            success: true,
            error_message:  "".to_string(),
        };
        let response = dbase::MotdResponse {
            status: Some(status),
            message: format!("Hello motd!"),
        };
        Ok(Response::new(response))
    }

    async fn getuser(
        &self,
        request: Request<dbase::GetUserRequest>,
    ) -> Result<Response<dbase::GetUserResponse>, Status> {
        println!("Received request from: {:?}", request);

        let status = dbase::DbaseStatus {
            success: false,
            error_message: "user failed".to_string(),
        };
        /*
        let userinfo = dbase.UserInfo (
            userid:   None(),
            username: None(),
            password: None(),
            aliasname: None(),
            phonenumber: None(),
            isadminuser: false,
            emailaddress: None(),
        );

         */
        let response = dbase::GetUserResponse {
            status: Some(status),
            userinfo: None,
        };
        Ok(Response::new(response))
    }

    async fn setuser(
        &self,
        request: Request<dbase::SetUserRequest>,
    ) -> Result<Response<dbase::DbaseStatus>, Status> {
        println!("Received request from: {:?}", request);

        let response = dbase::DbaseStatus {
            success: true,
            error_message: "".to_string(),
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

async fn stop_database(reason: &String) -> DbaseStatus {
    let response = dbase::DbaseStatus {
        success: true,
        error_message: reason.to_string(),
    };
    response
}