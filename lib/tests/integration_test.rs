//! Integration Tests

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[test]
fn log_test() {
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
}
