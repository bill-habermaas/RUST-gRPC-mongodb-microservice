
use bson::Document;
use tonic::{transport::Server, Request, Response, Status};
extern crate mongodb;
use mongodb::{Client, options::{ClientOptions}};
use mongodb::Collection;
use dbase::dbase_server::{Dbase, DbaseServer};
use dbase::UserInfo;
use crate::dbase::{DbaseStatus, GetUserResponse, SetUserRequest};
use mongodb::Database;
use mongodb::bson::doc;

mod motd;
mod util;
mod users;

//Todo Add match/err handling for all status responses
//Todo Add setting of MOTD
//Todo Finish getMOTD
//Todo code for user update
//Todo code for user delete
//Todo generate ObjectId used for all operations
//Todo Add time stamps to user and motd records
//Todo Prevent duplicate user records
//Todo Move all user database code to a separate package
//Todo Add apache license headers to each source module
//Todo setup motd seperate handler

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

    async fn getmotd(&self, request: Request<dbase::GetMotdRequest>,
        ) -> Result<Response<dbase::GetMotdResponse>, Status> {
        let req = request.into_inner();
        let response = motd::handle_getmotd(req.motd_filter).await;
        Ok(Response::new(response))
    }

    async fn getuser(&self,
        request: Request<dbase::GetUserRequest>,) -> Result<Response<dbase::GetUserResponse>, Status> {
        let req = request.into_inner();
        let response = handle_getuser(&req.username).await;
        Ok(Response::new(response))
    }

    async fn setuser(&self,
        request: Request<dbase::SetUserRequest>,) -> Result<Response<dbase::DbaseStatus>, Status> {
        let req = request.into_inner();
        let response = handle_setuser(&req).await;
        Ok(Response::new(response))
    }

    async fn deluser(&self, request: Request<dbase::DelUserRequest>,) -> Result<Response<dbase::DbaseStatus>, Status> {
        let req = request.into_inner();
        let response = users::handle_deluser(&req.username).await;
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

// Orderly termination of the database
async fn stop_database(reason: &String) -> DbaseStatus {
    let response = dbase::DbaseStatus {
        success: true,
        error_message: reason.to_string(),
    };
    response
}

// Find the user and return all record fields
async fn handle_getuser (username: &String) -> GetUserResponse {
    let mut status = dbase::DbaseStatus {
        success: false,
        error_message: "database not initialized".to_string(),
    };

    if MONGODB.get().is_some() == true {
        let filter = doc!("username": username);
        let db = MONGODB.get();
        let col: Collection<Document> = db.unwrap().collection("users");

        let result = col.find_one(Some(filter), None).await;

        println!("{:?}", &result);
        match result {
            Ok(result) => {
                let document: Document = result.unwrap();
                let userinfo = dbase::UserInfo {
                    userid: document.get("_id").unwrap().as_object_id().unwrap().to_hex(),
                    username: document.get("username").unwrap().as_str().unwrap().to_string(),
                    password: document.get("password").unwrap().as_str().unwrap().to_string(),
                    aliasname: document.get("aliasname").unwrap().as_str().unwrap().to_string(),
                    phonenumber: document.get("phonenumber").unwrap().as_str().unwrap().to_string(),
                    role: document.get("role").unwrap().as_str().unwrap().to_string(),
                    emailaddress: document.get("emailaddress").unwrap().as_str().unwrap().to_string(),
                };
                status.success = true;
                status.error_message = "".to_string();
                let response = dbase::GetUserResponse {
                    status: Some(status),
                    userinfo: Some(userinfo),
                };
                return response;
            }
            Err(e) => {
                status.success = false;
                status.error_message = e.to_string();
            },
        };
    }
    let response = dbase::GetUserResponse {
        status: Some(status),
        userinfo: None,
    };
    response
}

// Create a new user record
async fn handle_setuser(req: &SetUserRequest) -> DbaseStatus {
    let userinfo: UserInfo = req.userinfo.clone().unwrap();
    let doc = doc!(
        "username": userinfo.username,
        "password": userinfo.password,
        "aliasname": userinfo.aliasname,
        "phonenumber": userinfo.phonenumber,
        "role": userinfo.role,
        "emailaddress": userinfo.emailaddress,
        );
    let db = MONGODB.get();
    let col = db.unwrap().collection("users");
    col.insert_one(doc, None).await.unwrap();

    let response = dbase::DbaseStatus {
        success: true,
        error_message: "".to_string(),
    };
    response
}

/*
// Delete a user record
async fn handle_deluser(username: &String) -> DbaseStatus {
    let mut response = dbase::DbaseStatus {
        success: true,
        error_message: "".to_string(),
    };
    let db = MONGODB.get();
    let col: Collection<Document> = db.unwrap().collection("users");
    let filter = doc!("username": username);
    let _ = match col.delete_one(filter, None).await {
        Err(e) => {
            response.success = false;
            response.error_message = e.to_string();
        },
        Ok(r) => {
            if r.deleted_count == 0 {
                response.success = false;
                response.error_message = "nothing deleted".to_string();
            }
        },
    };
    response
}
*/