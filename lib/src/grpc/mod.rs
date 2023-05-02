#![doc = include_str!("README.md")]

use futures::lock::Mutex;
use std::sync::Arc;
use tonic::{async_trait, Status};

#[cfg(any(feature = "grpc_mock", test))]
pub mod mock;

#[cfg(not(test))]
use arrow_macros_derive::log_macros;
#[cfg(not(test))]
use log;
#[cfg(not(test))]
log_macros!("grpc", "app::grpc::clients");

#[cfg(test)]
use log::{debug as grpc_debug, error as grpc_error, info as grpc_info, warn as grpc_warn};

/// Generic gRPC Client trait to let the [`Client<T>`] trait know that `T` has a `connect` function
#[async_trait]
pub trait ClientConnect<T>
where
    Self: Sized + Client<T>,
    T: Send + Clone,
{
    /// wrapper for gRPC client connect function
    async fn connect(&self) -> Result<T, tonic::transport::Error>;

    /// Get a copy of the connected client
    async fn get_client(&self) -> Result<T, Status> {
        grpc_info!("(get_client) {} entry.", self.get_name());

        let arc = Arc::clone(self.get_inner());
        let mut client_option = arc.lock().await;

        // if already connected, return the client, else, try connect
        match &mut *client_option {
            Some(client) => {
                grpc_debug!(
                    "(get_client) already connected to {} server at {}. Returning cloned client.",
                    self.get_name(),
                    self.get_address()
                );
                Ok(client.clone())
            }
            None => {
                grpc_warn!("(get_client) client not connected yet.");
                grpc_info!(
                    "(get_client) connecting to {} server at {}.",
                    self.get_name(),
                    self.get_address()
                );

                match self.connect().await {
                    Ok(client) => {
                        grpc_info!(
                            "(get_client) success: connected to {} server at {}.",
                            self.get_name(),
                            self.get_address()
                        );
                        *client_option = Some(client.clone());
                        Ok(client)
                    }
                    Err(e) => {
                        let error = format!(
                            "(get_client) couldn't connect to {} server at {}; {}.",
                            self.get_name(),
                            self.get_address(),
                            e
                        );
                        grpc_error!("{}", error);
                        Err(Status::internal(error))
                    }
                }
            }
        }
    }
}

/// Generic gRPC Client trait to provide wrapper for [`GrpcClient`] struct creation
#[async_trait]
pub trait Client<T>
where
    T: Send + Clone,
{
    /// Invalidates the client if set
    async fn invalidate(&mut self);

    /// Create new [`GrpcClient`]
    fn new_client(server_host: &str, server_port: u16, name: &str) -> GrpcClient<T> {
        let opt: Option<T> = None;
        GrpcClient {
            inner: Arc::new(Mutex::new(opt)),
            address: format!("http://{server_host}:{server_port}"),
            name: name.to_string(),
        }
    }

    /// Get name string for client
    fn get_name(&self) -> String;

    /// Get connection string for client
    fn get_address(&self) -> String;

    /// Get GrpcClient inner value
    fn get_inner(&self) -> &Arc<Mutex<Option<T>>>;
}

/// Wrapper struct for our gRPC clients
#[derive(Debug, Clone)]
pub struct GrpcClient<T> {
    inner: Arc<Mutex<Option<T>>>,
    address: String,
    name: String,
}

#[async_trait]
impl<T> Client<T> for GrpcClient<T>
where
    T: Send + Clone,
{
    async fn invalidate(&mut self) {
        let arc = Arc::clone(&self.inner);
        let mut client = arc.lock().await;
        *client = None;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_address(&self) -> String {
        self.address.clone()
    }
    fn get_inner(&self) -> &Arc<Mutex<Option<T>>> {
        &self.inner
    }
}

/// Returns a ([String], [i32]) host, port from provided environment variables.
/// Returns default values (localhost, 50051) if environment variable is not found.
pub fn get_endpoint_from_env(env_host: &str, env_port: &str) -> (String, u16) {
    grpc_debug!("(get_endpoint_from_env) entry");

    let host = match std::env::var(env_host) {
        Ok(val) => val,
        Err(_) => {
            grpc_error!(
                "(get_endpoint_from_env) {} undefined, using default [localhost].",
                env_host
            );
            "localhost".to_string()
        }
    };
    let port: u16 = match std::env::var(env_port) {
        Ok(val) => match val.parse::<u16>() {
            Ok(val) => val,
            Err(_) => {
                grpc_error!(
                    "(get_endpoint_from_env) {} is not a valid u16 type, using default [50051].",
                    env_port
                );
                50051
            }
        },
        Err(_) => {
            grpc_error!(
                "(get_endpoint_from_env) {} undefined, using default [50051].",
                env_port
            );
            50051
        }
    };

    grpc_info!("(get_endpoint_from_env) host [{}], port [{}].", host, port);
    (host, port)
}

#[macro_export]
/// Implements [`ClientConnect`] trait for provided gRPC Client
macro_rules! grpc_client {
    ($rpc_service_client: ident) => {
        #[async_trait]
        impl $crate::grpc::ClientConnect<$rpc_service_client<Channel>>
            for $crate::grpc::GrpcClient<$rpc_service_client<Channel>>
        {
            /// Get a connected client object
            async fn connect(
                &self,
            ) -> Result<$rpc_service_client<Channel>, tonic::transport::Error> {
                $rpc_service_client::connect(self.get_address()).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::mock::*;
    use super::*;
    use tonic::{transport::Channel, Request, Response};

    pub mod grpc_client {
        #![allow(unused_qualifications)]
        include!("mock/grpc_client.rs");
    }
    use grpc_client::{client::MockClient, ReadyRequest};
    pub mod grpc_server {
        #![allow(unused_qualifications)]
        include!("mock/grpc_server.rs");
    }
    use grpc_server::server::{MockServer, MockService};
    grpc_client!(MockClient);

    /// Mock struct to implement our MockService for success tests
    #[derive(Default, Debug, Clone, Copy)]
    pub struct GrpcMockSuccess {}

    #[tonic::async_trait]
    impl MockService for GrpcMockSuccess {
        async fn is_ready(
            &self,
            request: Request<grpc_server::ReadyRequest>,
        ) -> Result<Response<grpc_server::ReadyResponse>, Status> {
            println!("Got a request: {:?}", request);
            let reply = grpc_server::ReadyResponse { ready: true };
            Ok(Response::new(reply))
        }
    }

    /// Mock struct to implement our MockService for fail tests
    #[derive(Default, Debug, Clone, Copy)]
    pub struct GrpcMockFail {}

    #[tonic::async_trait]
    impl MockService for GrpcMockFail {
        async fn is_ready(
            &self,
            request: Request<grpc_server::ReadyRequest>,
        ) -> Result<Response<grpc_server::ReadyResponse>, Status> {
            println!("Got a request: {:?}", request);
            Err(Status::internal("Mock not ready."))
        }
    }

    impl PartialEq for MockClient<Channel> {
        fn eq(&self, other: &Self) -> bool {
            self == other
        }
    }

    #[test]
    fn test_get_endpoint_from_env() {
        // test_get_endpoint_from_env_with_defaults
        std::env::remove_var("GRPC_PORT");
        std::env::remove_var("GRPC_HOST");
        let (server_host, server_port) = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");
        assert_eq!(server_host, "localhost");
        assert_eq!(server_port, 50051);

        // test_get_endpoint_from_env_with_valid_env_vars
        std::env::set_var("GRPC_PORT", "50055");
        std::env::set_var("GRPC_HOST", "custom_host");
        let (server_host, server_port) = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");
        assert_eq!(server_host, "custom_host");
        assert_eq!(server_port, 50055);

        // test_get_endpoint_from_env_with_invalid_port
        std::env::set_var("GRPC_PORT", "invalid");
        let (server_host, server_port) = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");
        assert_eq!(server_host, "custom_host");
        assert_eq!(server_port, 50051);
    }

    #[tokio::test]
    async fn test_mock_client_new_client() {
        let name = "mock_client";
        let server_host = "localhost";
        let server_port = 50050;

        let client: GrpcClient<MockClient<Channel>> =
            GrpcClient::new_client(server_host, server_port, name);

        assert_eq!(client.name, name);
        assert_eq!(
            client.address,
            format!("http://{}:{}", server_host, server_port)
        );
    }

    #[tokio::test]
    async fn test_grpc_client_server_not_ready() {
        let name = "test_client";
        let server_host = "localhost";
        let server_port = 50051;

        grpc_info!("ensure_server_running");

        let service = GrpcMockFail::default();
        let server_started = start_server(
            &format!("0.0.0.0:{}", server_port),
            MockServer::new(service),
        )
        .await;
        assert!(server_started.is_ok());
        grpc_info!("Server started");
        let shutdown_tx = server_started.unwrap();

        let mock_client: GrpcClient<MockClient<Channel>> =
            GrpcClient::new_client(server_host, server_port, name);

        // First time get_client, should create a new connection
        // See if we can send a request
        let result = mock_client.get_client().await;
        assert!(result.is_ok());

        // See if we can send a request
        let result = result
            .unwrap()
            .is_ready(Request::new(ReadyRequest {}))
            .await;
        grpc_debug!("{:?}", result);
        assert!(result.is_err());
        if let Err(result) = result {
            grpc_debug!("{}", result.message());
            assert_eq!(result.message(), "Mock not ready.");
        }
        // Send server the shutdown request
        shutdown_tx.send(()).expect("Could not stop server.");
    }

    // Running all tests that require a running server at once.
    // We need a known Channel for the ClientConnect implementation.
    #[tokio::test]
    async fn test_grpc_client_connected() {
        let name = "test_client";
        let server_host = "localhost";
        let server_port = 50052;

        grpc_info!("ensure_server_running");

        let service = GrpcMockSuccess::default();
        let server_started = start_server(
            &format!("0.0.0.0:{}", server_port),
            MockServer::new(service),
        )
        .await;
        assert!(server_started.is_ok());
        grpc_info!("Server started");
        let shutdown_tx = server_started.unwrap();

        let mock_client: GrpcClient<MockClient<Channel>> =
            GrpcClient::new_client(server_host, server_port, name);

        // First time get_client, should create a new connection
        // See if we can send a request
        let result = mock_client.get_client().await;
        assert!(result.is_ok());

        // See if we can send a request
        let result = result
            .unwrap()
            .is_ready(Request::new(ReadyRequest {}))
            .await;
        grpc_debug!("{:?}", result);
        assert!(result.is_ok());

        // Second time get_client, should already be connected
        let client = mock_client.get_client().await;
        assert!(client.is_ok());

        // See if we can send a request
        let result = client
            .unwrap()
            .is_ready(Request::new(ReadyRequest {}))
            .await;
        grpc_debug!("{:?}", result);
        assert!(result.is_ok());

        // Send server the shutdown request
        shutdown_tx.send(()).expect("Could not stop server.");
        // Give the server some time to shut down
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Third time get_client, should not be connected
        let client = mock_client.get_client().await;
        assert!(client.is_ok());
        // See if we get an error when server unreachable
        let result = client
            .unwrap()
            .is_ready(Request::new(ReadyRequest {}))
            .await;
        grpc_debug!("{:?}", result);
        assert!(result.is_err());

        // See if we can recover from a server reconnect
        grpc_info!("ensure_server_running");
        let server_started = start_server(
            &format!("0.0.0.0:{}", server_port),
            MockServer::new(service),
        )
        .await;
        assert!(server_started.is_ok());
        grpc_info!("Server started");
        let shutdown_tx = server_started.unwrap();

        // Fourth time get_client, should be connected again
        let client = mock_client.get_client().await;
        assert!(client.is_ok());
        // See if we get an answer from the server again
        let result = client
            .unwrap()
            .is_ready(Request::new(ReadyRequest {}))
            .await;
        grpc_debug!("{:?}", result);
        assert!(result.is_ok());

        // Send server the shutdown request
        shutdown_tx.send(()).expect("Could not stop server.");
    }

    #[tokio::test]
    async fn test_grpc_client_invalidate() {
        let name = "test_client";
        let server_host = "localhost";
        let server_port = 50053;

        let mut mock_client: GrpcClient<MockClient<Channel>> =
            GrpcClient::new_client(server_host, server_port, name);

        mock_client.invalidate().await;

        let arc = Arc::clone(&mock_client.inner);
        let client_option = arc.lock().await;
        assert_eq!(&*client_option, &None);
    }

    #[tokio::test]
    async fn test_grpc_server_address_parse_error() {
        let server_host = "invalid";
        let server_port = 50054;
        let service = GrpcMockSuccess::default();
        let server_started = start_server(
            &format!("{}:{}", server_host, server_port),
            MockServer::new(service),
        )
        .await;
        assert!(server_started.is_err());
    }

    #[tokio::test]
    async fn test_grpc_server_port_already_in_use() {
        let server_host = "0.0.0.0";
        let server_port = 50055;
        let service = GrpcMockSuccess::default();

        let server_started = start_server(
            &format!("{}:{}", server_host, server_port),
            MockServer::new(service),
        )
        .await;
        assert!(server_started.is_ok());
        grpc_info!("Server started");
        let shutdown_tx = server_started.unwrap();

        let result = start_server::<MockServer<GrpcMockSuccess>>(
            &format!("{}:{}", server_host, server_port),
            MockServer::new(service),
        )
        .await;
        print!("{:?}", result);
        assert!(result.is_err());

        // Send server the shutdown request
        shutdown_tx.send(()).expect("Could not stop server.");
    }
}
