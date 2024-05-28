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
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::debug!(target: concat!("app", "::", "example"), "({name}) {}", content)
            }};
        }
        #[doc = concat!("Writes a ", stringify!(info), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_info {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::info!(target: concat!("app", "::", "example"), "({name}) {}", content)
            }};
        }
        #[doc = concat!("Writes a ", stringify!(warn), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_warn {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::warn!(target: concat!("app", "::", "example"), "({name}) {}", content)
            }};
        }
        #[doc = concat!("Writes a ", stringify!(error), "! message to the `", "app", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_error {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::error!(target: concat!("app", "::", "example"), "({name}) {}", content)
            }};
        }
    );
    let output = log_macros_core(input.into());
    assert_tokens_eq(&output, &expected_output);
}

#[test]
#[should_panic]
fn test_log_macros_core_empty() {
    let input = TokenStream::new();
    log_macros_core(input);
}

#[test]
#[should_panic]
fn test_log_macros_core_parse_too_many_args() {
    let input = quote!("too", "many", "args");
    let result = log_macros_core(input);
    assert_tokens_eq(&result, &quote!());
}

#[test]
fn test_log_macros_core_two_args() {
    let input = quote!("example", "hello");

    let expected_output = quote!(
        #[doc = concat!("Writes a ", stringify!(debug), "! message to the `", "hello", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_debug {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::debug!(target: concat!("hello", "::", "example"), "({name}) {}", content)
            }};
        }
        #[doc = concat!("Writes a ", stringify!(info), "! message to the `", "hello", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_info {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::info!(target: concat!("hello", "::", "example"), "({name}) {}", content)
            }};
        }
        #[doc = concat!("Writes a ", stringify!(warn), "! message to the `", "hello", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_warn {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::warn!(target: concat!("hello", "::", "example"), "({name}) {}", content)
            }};
        }
        #[doc = concat!("Writes a ", stringify!(error), "! message to the `", "hello", "::", "example", "` logger")]
        #[macro_export]
        macro_rules! example_error {
            ($($arg:tt)+) => {{
                let content = format!($($arg)+);
                let name: &str = {{
                    fn f() {}

                    fn type_name_of<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }

                    type_name_of(f) // mod::path::target_function::f
                        .trim_end_matches("::f") // mod::path::target_function
                        .trim_end_matches(r#"::{{closure}}"#) // if {{closure}} is in the path
                        .split("::") // ["mod", "path", "target_function"]
                        .last() // "target_function"
                        .unwrap_or("unknown_function")
                }};
                log::error!(target: concat!("hello", "::", "example"), "({name}) {}", content)
            }};
        }
    );
    let output = log_macros_core(input.into());
    assert_tokens_eq(&output, &expected_output);
}
