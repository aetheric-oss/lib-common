# gRPC Client helper `Traits` and `Macros`

<p style="background:rgba(255,181,77,0.16);padding:0.75em;">
Only available with <code style="background:rgba(41,24,0,0.1);">grpc</code> feature enabled
</p>

Example usage:
```rust
use lib_common::grpc::*;
use lib_common::grpc_client;
use tonic::{transport::Channel, Request};
use tonic::async_trait;

pub mod grpc_client {
    #![allow(unused_qualifications)]
    include!("mock/grpc_client.rs");
}
use grpc_client::{client::MockClient, ReadyRequest};

// Call gRPC client macro provided by lib_common::grpc.
// This will implement the [`ClientConnect`] trait for the [`GrpcClient<MockClient<Channel>>`] object.
grpc_client!(MockClient);

#[tokio::main]
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port) = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

    // Initialize a new GrpcClient providing the connection host and port and a name to identify the client
    let grpc_client = GrpcClient::<MockClient<Channel>>::new_client(
        &host,
        port,
        "my_client",
    );

    // Get a client connection
    let mut connection = grpc_client.get_client().await?;

    // Call the gRPC functions
    let result = connection.is_ready(Request::new(ReadyRequest {})).await?;

    Ok(())
}
```
