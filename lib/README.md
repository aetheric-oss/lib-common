![Aetheric Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# `lib-common` Library

![GitHub stable release (latest by date)](https://img.shields.io/github/v/release/aetheric-oss/lib-common?sort=semver&color=green)
![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/aetheric-oss/lib-common?include_prereleases)
[![Coverage Status](https://coveralls.io/repos/github/aetheric-oss/lib-common/badge.svg?branch=develop)](https://coveralls.io/github/aetheric-oss/lib-common)
![Sanity Checks](https://github.com/aetheric-oss/lib-common/actions/workflows/sanity_checks.yml/badge.svg?branch=main)
![Rust Checks](https://github.com/aetheric-oss/lib-common/actions/workflows/rust_ci.yml/badge.svg?branch=main)
![Python PEP8](https://github.com/aetheric-oss/lib-common/actions/workflows/python_ci.yml/badge.svg?branch=main)
![Arrow DAO
Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## Overview

Common functions and data types across the Arrow microservices.

### Feature flags

#### grpc
<p style="background:rgba(255,181,77,0.16);padding:0.75em;">
The <code style="background:rgba(41,24,0,0.1);">grpc</code> module is always enabled for Rust tests.
</p>

Enabling the `grpc` feature will add the following dependencies:
- `tonic`
- `futures`
- `log`
- `prost`

Enabling the `grpc` module gives access to the [`Client`](grpc::Client) and
[`ClientConnect`](grpc::ClientConnect) Traits, in addition to the [`GrpcClient`](grpc::GrpcClient) Struct.


#### grpc_mock
<p style="background:rgba(255,181,77,0.16);padding:0.75em;">
The <code style="background:rgba(41,24,0,0.1);">grpc_mock</code> module is always enabled for Rust tests.
</p>

Enabling the `grpc_mock` feature will add the following dependencies:
- `grpc`
- `tokio`
- `tower`
- `http`
- `hyper`

Enabling the `grpc_mock` module gives access to a
[`start_server`](grpc::mock::start_server) function which provides a wrapper to
start any gRPC server implementation in a non-blocking thread. This can be used
when running unit tests.
