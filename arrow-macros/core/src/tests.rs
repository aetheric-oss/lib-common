#![cfg(test)]

use crate::log_macros_core;
use proc_macro2::TokenStream;
use quote::quote;

#[cfg(not(tarpaulin_include))]
// no_coverage: Helper test function
fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}

#[test]
fn test_log_macro_default_prefix() {
    let input = quote!("example");

    let expected_output = quote!(
        #[doc = concat!("Writes a ", stringify!(debug), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_debug {
            ($($arg:tt)+) => {
                log::debug!(target: concat!("app", "::", "example"), $($arg)+)
            };
        }
        #[doc = concat!("Writes a ", stringify!(info), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_info {
            ($($arg:tt)+) => {
                log::info!(target: concat!("app", "::", "example"), $($arg)+)
            };
        }
        #[doc = concat!("Writes a ", stringify!(warn), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_warn {
            ($($arg:tt)+) => {
                log::warn!(target: concat!("app", "::", "example"), $($arg)+)
            };
        }
        #[doc = concat!("Writes a ", stringify!(error), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_error {
            ($($arg:tt)+) => {
                log::error!(target: concat!("app", "::", "example"), $($arg)+)
            };
        }
    );
    let output = log_macros_core(input.into());
    assert_tokens_eq(&output, &expected_output);
}
