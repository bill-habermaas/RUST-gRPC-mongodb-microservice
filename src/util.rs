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

use bson::oid::ObjectId;
use crate::dbase;
use crate::dbase::{DbaseStatus};

// Create a DbaseStatus response block
pub fn makestatus(success: bool, error_message: String) -> DbaseStatus {
    let status = dbase::DbaseStatus {
        success: success,
        error_message: error_message,
    };
    status
}

// Convert a hexidecimal userid into an IbjectId
use std::str::FromStr;
pub fn create_objectid(hexid: String) -> ObjectId {
    let objid = mongodb::bson::oid::ObjectId::from_str(&hexid).unwrap();
    objid
}
