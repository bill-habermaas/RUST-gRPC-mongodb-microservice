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
