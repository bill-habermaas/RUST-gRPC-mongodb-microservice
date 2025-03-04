/*
 * Copyright 2025 Habermaas Systems, Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 *  express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//use tonic::Response;
use dbase::dbase_client::DbaseClient;
use dbase::DbinitRequest;
use dbase::SetUserRequest;
use dbase::GetUserRequest;
use dbase::GetMotdRequest;
use dbase::CheckUserRequest;
use crate::dbase::{DelUserRequest, UserInfo};

pub mod dbase {
    tonic::include_proto!("dbase");
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DbaseClient::connect("http://[::1]:50052").await?;

    let request = tonic::Request::new(DbinitRequest {
        dbspec: "mongodb://localhost:27017".to_string(),
        dbname: "test".to_string(),
    });
    println!("Sending request to gRPC Server...");
    let response = client.dbinit(request).await?;
    println!("dbinit response={:?}\n", response);

    let request = tonic::Request::new(CheckUserRequest{
        username: "bill".to_string(),
    });
    let response = client.chkuser(request).await;
    println!("chkuser bill should fail -- {:?}\n", response);

    let userinfo: UserInfo = dbase::UserInfo {
        userid: "".to_string(),
        username: "bill".to_string(),
        password: "password".to_string(),
        aliasname: "willi".to_string(),
        phonenumber: "631-252-4737".to_string(),
        role: "user".to_string(),
        emailaddress: "bill@habermaas.us".to_string(),
    };
    let uinfo = userinfo.clone();

    let request = tonic::Request::new(SetUserRequest {
        userinfo: Some(userinfo),
    });
    let response = client.setuser(request).await;
    println!("set user should succeed\n{:?}\n", response);

    let request = tonic::Request::new(SetUserRequest {
        userinfo: Some(uinfo),
    });
    let response = client.setuser(request).await;
    println!("set user should fail - duplicate\n{:?}\n", response);

    let request = tonic::Request::new(GetUserRequest {
        username: "bill".to_string(),
    });
    let response= client.getuser(request).await;
    println!("getuser should succeed\n{:?}\n", response);

    let request = tonic::Request::new(CheckUserRequest{
        username: "bill".to_string(),
    });
    let response = client.chkuser(request).await;
    println!("chkuser bill should succeed\n{:?}\n", response);

    let request = tonic::Request::new(GetUserRequest {
        username: "fred".to_string(),
    });
    let response= client.getuser(request).await;
    println!("getuser fred should fail\n{:?}\n", response);

    let request = tonic::Request::new(DelUserRequest {
        username:"bill".to_string(),
    });
    let response = client.deluser(request).await?;
    println!("deluser should succeed\n{:?}\n", response);

    let req = tonic::Request::new(GetMotdRequest {
        motd_filter: "filter".to_string(),
    });
    let resp = client.getmotd(req).await?;
    println!("getmotd {:?}\n", resp);
    Ok(())
}
