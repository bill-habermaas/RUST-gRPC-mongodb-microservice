
extern crate mongodb;

use bson::Document;
use mongodb::Collection;
use mongodb::bson::doc;

use crate::{dbase, MONGODB};
use crate::dbase::{DbaseStatus};

// Delete a user record
pub async fn handle_deluser(username: &String) -> DbaseStatus {
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
