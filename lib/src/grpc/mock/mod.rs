#![doc = include_str!("README.md")]

use std::convert::Infallible;
use tokio::sync::oneshot::{self, Sender};
use tonic::body::BoxBody;
use tonic::transport::{NamedService, Server};
use tower::Service;

/// Starts a [`tokio`] server listening on 50051 for our `MockService`
/// # Example
/// ```
/// use lib_common::grpc::mock::start_server;
///
/// pub mod mock_server {
///     #![allow(unused_qualifications)]
///     include!("grpc_server.rs");
/// }
/// use mock_server::server::{MockService, MockServer};
/// use mock_server::*;
///
/// async fn start() {
///     let service = GrpcMockImpl::default();
///     let shutdown_tx = start_server("0.0.0.0:50051", MockServer::new(service)).await.expect("(start) Could not start server.");
///     // send server shutdown signal
///     shutdown_tx.send(()).expect("(start) Unable to shutdown server");
/// }
///
/// #[derive(Default, Debug, Clone, Copy)]
/// pub struct GrpcMockImpl {}
///
/// #[tonic::async_trait]
/// impl MockService for GrpcMockImpl {
///     async fn is_ready(
///         &self,
///         request: tonic::Request<ReadyRequest>,
///     ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
///         println!("(is_ready MOCK) Got a request: {:?}", request);
///         let reply = ReadyResponse { ready: true };
///         Ok(tonic::Response::new(reply))
///     }
/// }
/// ```
pub async fn start_server<S>(
    addr: &str,
    service: S,
) -> Result<Sender<()>, Box<dyn std::error::Error>>
where
    S: Service<http::Request<hyper::Body>, Response = http::Response<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    println!("(start_server) Starting server on {}", addr);

    // Create channels to send shutdown event
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let addr = match addr.parse() {
        Ok(addr) => Ok(addr),
        Err(e) => {
            eprintln!("(start_server) Parse error: {e}");
            Err(e)
        }
    }?;

    let result = tokio::spawn(async move {
        let server = Server::builder()
            .add_service(service)
            .serve_with_shutdown(addr, async {
                shutdown_rx.await.ok();
            });

        if let Err(err) = server.await {
            eprintln!("(start_server) Server error: {:?}", err);
            panic!("error");
        }
    });

    // Server takes some time to start, so wait a bit for that
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // Check if the server is still running
    if result.is_finished() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Server stopped!?!?",
        )));
    }

    Ok(shutdown_tx)
}

/// Starts a [`tokio`] server listening on the provided [`DuplexStream`](tokio::io::DuplexStream)
///
/// # Example
///
/// ```
/// use lib_common::grpc::mock::start_mock_server;
///
/// pub mod mock_server {
///     #![allow(unused_qualifications)]
///     include!("grpc_server.rs");
/// }
/// use mock_server::server::{MockService, MockServer};
/// use mock_server::*;
///
/// async fn start() {
///     let (client, server) = tokio::io::duplex(1024);
///     let service = GrpcMockImpl::default();
///     start_mock_server(server, MockServer::new(service)).await.expect("Could not start server.");
/// }
///
/// #[derive(Default, Debug, Clone, Copy)]
/// pub struct GrpcMockImpl {}
///
/// #[tonic::async_trait]
/// impl MockService for GrpcMockImpl {
///     async fn is_ready(
///         &self,
///         request: tonic::Request<ReadyRequest>,
///     ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
///         println!("(is_ready MOCK) Got a request: {:?}", request);
///         let reply = ReadyResponse { ready: true };
///         Ok(tonic::Response::new(reply))
///     }
/// }
/// ```
pub async fn start_mock_server<S>(
    server: tokio::io::DuplexStream,
    service: S,
) -> Result<(), tonic::transport::Error>
where
    S: Service<http::Request<hyper::Body>, Response = http::Response<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    println!("(start_mock_server) Starting server on {:?}", server);

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(service)
            .serve_with_incoming(futures::stream::iter(vec![Ok::<_, std::io::Error>(server)]))
            .await
    });

    Ok(())
}

#[macro_export]
/// Implements [`ClientConnect`](super::ClientConnect) trait for provided gRPC Client
/// Starts a mock server and creates a connection using a duplex channel
macro_rules! grpc_mock_client {
    ($rpc_service_client:ident, $rpc_service_server:ident, $rpc_service:ident) => {
        #[tonic::async_trait]
        impl $crate::grpc::ClientConnect<$rpc_service_client<Channel>>
            for $crate::grpc::GrpcClient<$rpc_service_client<Channel>>
        {
            /// Get a connected client object
            async fn connect(
                &self,
            ) -> Result<$rpc_service_client<Channel>, tonic::transport::Error> {
                let (client, server) = tokio::io::duplex(1024);
                let grpc_service = $rpc_service::default();
                lib_common::grpc::mock::start_mock_server(
                    server,
                    $rpc_service_server::new(grpc_service),
                )
                .await?;

                // Move client to an option so we can _move_ the inner value
                // on the first attempt to connect. All other attempts will fail.
                let mut client = Some(client);
                let channel = tonic::transport::Endpoint::try_from("http://[::]:50051")?
                    .connect_with_connector(tower::service_fn(move |_: tonic::transport::Uri| {
                        let client = client.take();

                        async move {
                            if let Some(client) = client {
                                Ok(client)
                            } else {
                                Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "Client already taken",
                                ))
                            }
                        }
                    }))
                    .await?;

                Ok($rpc_service_client::new(channel))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::grpc::{Client, ClientConnect, GrpcClient};
    use tonic::transport::Channel;
    pub mod mock_server {
        #![allow(unused_qualifications)]
        include!("grpc_server.rs");
    }
    use mock_server::server::{MockServer, MockService};
    use mock_server::*;

    #[derive(Default, Debug, Clone, Copy)]
    pub struct GrpcMockImpl {}

    #[tonic::async_trait]
    impl MockService for GrpcMockImpl {
        async fn is_ready(
            &self,
            request: tonic::Request<mock_server::ReadyRequest>,
        ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
            println!("(is_ready MOCK) Got a request: {:?}", request);
            let reply = ReadyResponse { ready: true };
            Ok(tonic::Response::new(reply))
        }
    }

    pub mod grpc_client {
        #![allow(unused_qualifications)]
        include!("grpc_client.rs");
    }
    use grpc_client::client::MockClient;
    use grpc_client::ReadyRequest;
    grpc_mock_client!(MockClient, MockServer, GrpcMockImpl);

    #[tokio::test]
    async fn test_mock_client_connect() {
        let name = "mock_client";
        let server_host = "localhost";
        let server_port = 50050;

        let client: GrpcClient<MockClient<Channel>> =
            GrpcClient::new_client(server_host, server_port, name);

        let connection = client.get_client(1).await;
        println!("{:?}", connection);
        assert!(connection.is_ok());

        // See if we can send a request
        let result = connection
            .clone()
            .unwrap()
            .is_ready(tonic::Request::new(ReadyRequest {}))
            .await;
        println!("{:?}", result);

        // See if we can send a second request
        let result = connection
            .unwrap()
            .is_ready(tonic::Request::new(ReadyRequest {}))
            .await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
