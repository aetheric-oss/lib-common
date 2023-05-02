//! Integration Tests

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[test]
fn log_test() {
    use lib_common::grpc::get_endpoint_from_env;
    use log;
    use logtest::Logger;

    // Start the logger.
    let mut logger = Logger::start();

    #[macro_use]
    mod test_macros {
        use lib_common::log_macros;
        log_macros!("test1");
        log_macros!("test2", "other::prefix");
    }

    test1_debug!("debug");
    assert_eq!(logger.pop().unwrap().args(), "debug");
    test1_debug!("debug {}", "param");
    assert_eq!(logger.pop().unwrap().args(), "debug param");

    test1_error!("error");
    assert_eq!(logger.pop().unwrap().args(), "error");
    test1_error!("error {}", "param");
    assert_eq!(logger.pop().unwrap().args(), "error param");

    test1_info!("info");
    assert_eq!(logger.pop().unwrap().args(), "info");

    test1_warn!("warn");
    assert_eq!(logger.pop().unwrap().args(), "warn");

    test2_debug!("debug {}", "test");
    assert_eq!(logger.pop().unwrap().args(), "debug test");

    test2_error!("error");
    assert_eq!(logger.pop().unwrap().args(), "error");

    test2_info!("info");
    assert_eq!(logger.pop().unwrap().args(), "info");

    test2_warn!("warn");
    assert_eq!(logger.pop().unwrap().args(), "warn");

    // test_get_endpoint_from_env_with_defaults
    std::env::remove_var("GRPC_PORT");
    std::env::remove_var("GRPC_HOST");
    let _ = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");
    let _ = logger.pop();
    assert_eq!(
        logger.pop().unwrap().args(),
        "(get_endpoint_from_env) GRPC_HOST undefined, using default [localhost]."
    );
    assert_eq!(
        logger.pop().unwrap().args(),
        "(get_endpoint_from_env) GRPC_PORT undefined, using default [50051]."
    );
    assert_eq!(
        logger.pop().unwrap().args(),
        "(get_endpoint_from_env) host [localhost], port [50051]."
    );

    // test_get_endpoint_from_env_with_valid_env_vars
    // Set up env vars
    std::env::set_var("GRPC_PORT", "50055");
    std::env::set_var("GRPC_HOST", "custom_host");
    let _ = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");
    let _ = logger.pop();
    assert_eq!(
        logger.pop().unwrap().args(),
        "(get_endpoint_from_env) host [custom_host], port [50055]."
    );

    // test_get_endpoint_from_env_with_invalid_port
    // Set up env vars
    std::env::set_var("GRPC_PORT", "invalid");
    let _ = get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");
    let _ = logger.pop();
    assert_eq!(
        logger.pop().unwrap().args(),
        "(get_endpoint_from_env) GRPC_PORT is not a valid u16 type, using default [50051]."
    );
    assert_eq!(
        logger.pop().unwrap().args(),
        "(get_endpoint_from_env) host [custom_host], port [50051]."
    );
}
