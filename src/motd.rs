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
use crate::dbase;
use crate::dbase::{GetMotdResponse};
use crate::dbase::{SetMotdResponse};

use crate::util;

pub async fn handle_getmotd(motd_filter: String) -> GetMotdResponse {
    let _filter = motd_filter;

    let status = util::makestatus(false, "".to_string());
    let response = dbase::GetMotdResponse {
        status: Some(status),
        message: "motd response".to_string(),
    };
    response
}

pub async fn handle_setmotd(motd_filter: String) -> SetMotdResponse {
    let status = util::makestatus(false, "not supported".to_string());
    let response = dbase::SetMotdResponse {
        status: Some(status),
    };
    response
}