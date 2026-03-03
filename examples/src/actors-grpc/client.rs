use std::time::Duration;

use hello_world::{greeter_client::GreeterClient, HelloRequest};
use serde::{Deserialize, Serialize};
use tonic::metadata::MetadataValue;

pub mod hello_world {
    include!("../invoke/protos/helloworld.rs");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyResponse {
    pub available: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyRequest {
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allow time for the server and Dapr sidecars to become ready
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Define the Dapr address
    let addr = "https://127.0.0.1".to_string();

    // Create the Dapr client
    let mut client = dapr::Client::<dapr::client::TonicClient>::connect(addr).await?;

    // -- Test 1: Invoke actor via Dapr actor API --
    let data = MyRequest {
        name: "test".to_string(),
    };

    let resp: Result<MyResponse, dapr::error::Error> = client
        .invoke_actor("MyActor", "a1", "do_stuff", data, None)
        .await;

    println!("Actor response: {resp:#?}");

    // -- Test 2: Invoke gRPC service via Dapr gRPC proxy --
    let port: u16 = std::env::var("DAPR_GRPC_PORT").unwrap().parse().unwrap();
    let grpc_address = format!("https://127.0.0.1:{port}");

    let mut greeter_client = GreeterClient::connect(grpc_address).await?;

    let request = HelloRequest {
        name: "Test".to_string(),
    };
    let mut request = tonic::Request::new(request);
    request.metadata_mut().append(
        "dapr-app-id",
        MetadataValue::from_static("actors-grpc-server"),
    );

    let response = greeter_client.say_hello(request).await.unwrap();
    let hello_reply = response.into_inner();

    println!("gRPC response: {hello_reply:#?}");

    Ok(())
}
