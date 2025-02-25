use dbase::dbase_client::DbaseClient;
use dbase::ConnectRequest;

pub mod dbase {
    tonic::include_proto!("dbase");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DbaseClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(ConnectRequest {
        username: "Tonic".into(),
        password: "password".into(),
    });

    println!("Sending request to gRPC Server...");
    let response = client.connect(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}