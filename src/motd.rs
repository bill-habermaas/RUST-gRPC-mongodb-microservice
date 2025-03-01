
use crate::dbase;
use crate::dbase::{GetMotdResponse};

use crate::util;
//use crate::util::{makestatus};

pub async fn handle_getmotd(motd_filter: String) -> GetMotdResponse {
    let _filter = motd_filter;

    let status = util::makestatus(false, "".to_string());
    let response = dbase::GetMotdResponse {
        status: Some(status),
        message: "motd response".to_string(),
    };
    response
}