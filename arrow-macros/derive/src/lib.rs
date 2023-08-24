#![doc = include_str!("../README.md")]

use arrow_macros_core::log_macros_core;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// Generate log macros for debug, info, warn and error functions.
///
/// # Example:
/// ```
/// pub mod grpc {
///     //! Common Functions and Types Library for Arrow Services
///     use arrow_macros_derive::log_macros;
///     log_macros!("grpc");
/// }
/// ```
#[proc_macro_error]
#[proc_macro]
#[cfg(not(tarpaulin_include))]
// no_coverage: proc_macro's can not be tested, using log_macros_core for that
pub fn log_macros(input: TokenStream) -> TokenStream {
    log_macros_core(input.into()).into()
}
