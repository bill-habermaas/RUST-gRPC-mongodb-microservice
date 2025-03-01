//use tonic::Response;
use dbase::dbase_client::DbaseClient;
use dbase::DbinitRequest;
use dbase::SetUserRequest;
use dbase::GetUserRequest;
use dbase::GetUserResponse;
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

    let userinfo: UserInfo = dbase::UserInfo {
        userid: "0".to_string(),
        username: "bill".to_string(),
        password: "password".to_string(),
        aliasname: "willi".to_string(),
        phonenumber: "631-252-4737".to_string(),
        role: "user".to_string(),
        emailaddress: "bill@habermaas.us".to_string(),
    };
    let request = tonic::Request::new(SetUserRequest {
        userinfo: Some(userinfo),
    });
    let _response = client.setuser(request).await;
    //println!("{}\n", response);

    let request = tonic::Request::new(GetUserRequest {
        username: "bill".to_string(),
    });
    let response= client.getuser(request).await;
    let rr = response.unwrap().into_inner();
    handlegetuser(&rr);

    let request = tonic::Request::new(DelUserRequest {
        username:"bill".to_string(),
    });
    let response = client.deluser(request).await?;
    println!("deluser: {:?}", response);

    Ok(())
}

fn handlegetuser(rr: &GetUserResponse) {
    let userinfo: UserInfo = rr.userinfo.clone().unwrap();
    let user = userinfo.username;
    let pass = userinfo.password;
    println!("{} {}", user, pass);
}

