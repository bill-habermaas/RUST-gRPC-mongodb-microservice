
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