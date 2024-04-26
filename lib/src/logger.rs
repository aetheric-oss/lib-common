//! Logging utilities

use tokio::sync::OnceCell;

/// Initialized log4rs handle
static LOG_HANDLE: OnceCell<Option<log4rs::Handle>> = OnceCell::const_new();

/// Obtain the log handle for unit test purposes
pub async fn get_log_handle() -> Option<log4rs::Handle> {
    LOG_HANDLE
        .get_or_init(|| async move {
            // Set up basic logger to make sure we can write to stdout
            let stdout = log4rs::append::console::ConsoleAppender::builder()
                .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                    "{d(%Y-%m-%d %H:%M:%S)} | {I} | {h({l}):5.5} | {f}:{L} | {m}{n}",
                )))
                .build();

            let appender = log4rs::config::Appender::builder().build("stdout", Box::new(stdout));
            let root = log4rs::config::Root::builder()
                .appender("stdout")
                .build(log::LevelFilter::Debug);

            let config = log4rs::config::Config::builder()
                .appender(appender)
                .build(root)
                .ok()?;

            log4rs::init_config(config).ok()
        })
        .await
        .to_owned()
}

/// Initialize a log4rs logger with provided configuration file path
pub async fn load_logger_config_from_file(config_file: &str) -> Result<(), String> {
    let log_handle = get_log_handle()
        .await
        .ok_or("(load_logger_config_from_file) Could not get the log handle.")?;

    let config =
        log4rs::config::load_config_file(config_file, Default::default()).map_err(|e| {
            format!(
                "(logger) Could not parse log config file [{}]: {}.",
                config_file, e,
            )
        })?;

    log_handle.set_config(config);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug as ut_debug, error as ut_error, info as ut_info};

    #[tokio::test]
    async fn test_load_logger_config_from_file() {
        get_log_handle().await;
        ut_info!("(test_config_from_env) Start.");

        let result = load_logger_config_from_file("/usr/src/app/lib/tests/log4rs.yaml").await;
        ut_debug!("(test_config_from_env) {:?}", result);
        assert!(result.is_ok());

        // This message should be written to file
        ut_error!("(test_config_from_env) Testing log config from file. This should be written to the tests.log file.");

        ut_info!("(test_config_from_env) Success.");
    }
}
