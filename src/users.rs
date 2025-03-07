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
extern crate mongodb;

use std::collections::HashMap;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::{bson, Document};

use crate::{dbase, util, MONGODB};
use crate::dbase::{DbaseStatus, SetUserRequest, SetUserResponse, UserInfo};
use crate::dbase::GetUserResponse;
use crate::dbase::CheckUserResponse;
use crate::dbase::UpdateUserResponse;

// Find the user and return all record fields
pub async fn handle_getuser (username: &String) -> GetUserResponse {
    let mut status = util::makestatus(false, "database not initialized".to_string());

    if MONGODB.get().is_some() == true {
        let filter = doc!("username": username);
        let db = MONGODB.get();
        let col: Collection<Document> = db.unwrap().collection("users");
        match col.find_one(Some(filter), None).await {
            Ok(result) => {
                if result.is_none() {
                    status.error_message = "user does not exist".to_string();
                    let response = dbase::GetUserResponse {
                        status: Some(status),
                        userinfo: None,
                    };
                    return response;
                }
                let document: Document = result.unwrap();
                let userinfo = dbase::UserInfo {
                    userid: document.get("_id").unwrap().as_object_id().unwrap().to_string(),
                    username: document.get("username").unwrap().as_str().unwrap().to_string(),
                    password: document.get("password").unwrap().as_str().unwrap().to_string(),
                    aliasname: document.get("aliasname").unwrap().as_str().unwrap().to_string(),
                    phonenumber: document.get("phonenumber").unwrap().as_str().unwrap().to_string(),
                    role: document.get("role").unwrap().as_str().unwrap().to_string(),
                    emailaddress: document.get("emailaddress").unwrap().as_str().unwrap().to_string(),
                };
                status = util::makestatus(true, "".to_string());
                let response = dbase::GetUserResponse {
                    status: Some(status),
                    userinfo: Some(userinfo),
                };
                return response;
            }
            Err(e) => {
                status = util::makestatus(false, e.to_string());
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
pub async fn handle_setuser(req: &SetUserRequest) -> SetUserResponse {
    let userinfo: UserInfo = req.userinfo.clone().unwrap();
    let user = userinfo.username.clone();
    // make sure user does not exist so we don't create duplicates
    let chkresp = handle_chkuser(&user).await;
    let boolexists = chkresp.status.unwrap().success;
    if boolexists {
        let dupstatus = util::makestatus(false, "duplicate user".to_string());
        let dupresp = dbase::SetUserResponse {
            status: Some(dupstatus),
            userid: "".to_string(),
        };
        return dupresp;
    }
    let doc = doc!(
        "username": userinfo.username,
        "password": userinfo.password,
        "aliasname": userinfo.aliasname,
        "phonenumber": userinfo.phonenumber,
        "role": userinfo.role,
        "emailaddress": userinfo.emailaddress,
        );
    let db = MONGODB.get();
    let col: Collection<Document> = db.unwrap().collection("users");
    match col.insert_one(doc, None).await {
        Ok(r) => {
            let id = r.inserted_id.as_object_id().unwrap().to_hex().to_string();
            let status = util::makestatus(true, "".to_string());
            let response = dbase::SetUserResponse {
                status: Some(status),
                userid: id,
            };
            return response;
        },
        Err(e) => {
            let status = util::makestatus(false, e.to_string());
            let response = dbase::SetUserResponse {
                status: Some(status),
                userid: "".to_string(),
            };
            return response;
        },
    };
}

// Delete a user record
pub async fn handle_deluser(username: &String) -> DbaseStatus {
    let mut response = dbase::DbaseStatus {
        success: true,
        error_message: "".to_string(),
    };
    let db = MONGODB.get();
    let col: Collection<Document> = db.unwrap().collection("users");
    let filter = doc!("username": username);
    match col.delete_one(filter, None).await {
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

// Check if username exists
pub async fn handle_chkuser(username: &String) -> CheckUserResponse {
    let doc = doc!("username": username);
    let db = MONGODB.get();
    let col: Collection<Document> = db.unwrap().collection("users");
    let mut status = util::makestatus(true, "".to_string());
    let mut response = dbase::CheckUserResponse {
        status: Some(status.clone()),
        userid: "".to_string(),
    };
    match col.find_one(doc, None).await {
        Err(e) => {
            status.success = false;
            status.error_message = e.to_string();
        },
        Ok(result) => {
            if result.is_none() {
                let status = util::makestatus(false, "user does not exist".to_string());
                response.status = Some(status);
                return response;
            }
            let document: Document = result.unwrap();
            response.userid = document.get("_id").unwrap().as_object_id().unwrap().to_string();
            status.error_message = "username already exists".to_string();
        },
    };
    response.status = Some(status);
    response
}

pub async fn handle_upduser(username: String, mapfields: HashMap<String, String>) -> UpdateUserResponse {
    use mongodb::bson::Bson;
    let username = username;
    let mut status = util::makestatus(true, "".to_string());
    // have to convert HashMap<String,String> to HashMap<String,Bson>
    let mut mymap: HashMap<String, Bson> = HashMap::new();
    // Iterate through incoming map
    let mut bsonmap: HashMap<String, Bson> = HashMap::new();
    for (key, value) in mymap {
        bsonmap.insert(key.clone(), bson!(value.clone().to_string()));
    }
    let db = MONGODB.get();
    let col: Collection<Document> = db.unwrap().collection("users");
    let changes_document: Document = bsonmap.into_iter().collect();
    match col.update_one(
        doc! { "userName": username.to_string() },
        doc! { "$set": changes_document },
        None,
    ).await {
        Ok(r) => {
            //status.error_message = "".to_string();
        },
        Err(e) => { status.success = false; status.error_message = e.to_string() },
    }

    let response = dbase::UpdateUserResponse{
        status: Some(status),
    };
    response
}