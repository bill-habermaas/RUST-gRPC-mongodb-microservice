use crate::dbase;
use crate::dbase::{DbaseStatus};

pub fn makestatus(success: bool, error_message: String) -> DbaseStatus {
    let status = dbase::DbaseStatus {
        success: success,
        error_message: error_message,
    };
    status
}

