#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, Ident, LitStr, Token};

/// Generate debug, info, warn and error log macro's
/// ```compile_fail
/// fn test_log_macros_core_invalid_tokens() {
///     let input = quote! {
///         #[inert <T>]
///         struct S;
///     };
///    log_macros_core(input);
/// }
/// ```
pub fn log_macros_core(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        abort!(input, "log_macros takes at least 1 argument")
    }

    let input = match Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse2(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    let log_type;
    let mut log_prefix = String::from("app");
    if input.len() == 1 {
        log_type = input[0].value();
    } else if input.len() == 2 {
        log_type = input[0].value();
        log_prefix = input[1].value();
    } else {
        abort!(input, "log_macros expects either one or two arguments")
    }

    let macros = [
        Ident::new("debug", proc_macro2::Span::call_site()),
        Ident::new("info", proc_macro2::Span::call_site()),
        Ident::new("warn", proc_macro2::Span::call_site()),
        Ident::new("error", proc_macro2::Span::call_site()),
    ];

    let macro_exports = macros.iter().map(|level| {
        // this can panic with an invalid identifier
        let macro_name = syn::Ident::new(
            &format!("{}_{}", log_type, level),
            proc_macro2::Span::call_site(),
        );

        quote! {
            #[doc = concat!("Writes a ", stringify!(#level), "! message to the `", #log_prefix, "::", #log_type, "` logger")]
            #[macro_export]
            macro_rules! #macro_name {
                ($($arg:tt)+) => {
                    let content = format!($($arg)+);
                    let name = {
                        fn f() {}
                        fn type_name_of<T>(_: T) -> &'static str {
                            std::any::type_name::<T>()
                        }
                        let name = type_name_of(f);
                        name.strip_suffix("::f").unwrap_or("error_removing_suffix")
                    };
                    log::#level!(target: concat!(#log_prefix, "::", #log_type), "({}) {}", name, content)
                };
            }
        }
    });

    quote! {
        #(#macro_exports)*
    }
}
