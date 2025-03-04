/*
 * Copyright 2025 Habermaas Systems, Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
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
//Todo Add time stamps to user and motd records

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

    async fn setmotd(&self, request: Request<dbase::SetMotdRequest>,
    ) -> Result<Response<dbase::SetMotdResponse>, Status> {
        let req = request.into_inner();
        let response = motd::handle_setmotd(req.motd_filter).await;
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

    async fn upduser(&self, request: Request<dbase::UpdateUserRequest>,)
        -> Result<Response<dbase::UpdateUserResponse>, Status> {
        let req = request.into_inner();
        let response = users::handle_upduser(req.username, req.mapfields).await;
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