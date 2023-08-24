# gRPC Client and Server mock helpers

<p style="background:rgba(255,181,77,0.16);padding:0.75em;">
Only available with <code style="background:rgba(41,24,0,0.1);">grpc_mock</code> feature enabled
</p>

Provides access to a mock Server and Client which are mainly used for unit testing the `grpc` module itself.
The [`start_server`] function can be useful for external modules as well.
