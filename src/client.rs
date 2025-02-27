
use dbase::dbase_client::DbaseClient;
use dbase::DbinitRequest;

pub mod dbase {
    tonic::include_proto!("dbase");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DbaseClient::connect("http://[::1]:50052").await?;

    let request = tonic::Request::new(DbinitRequest {
        dbspec: "mongodb://localhost:27017".to_string(),
    });

    println!("Sending request to gRPC Server...");
    let response = client.dbinit(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}