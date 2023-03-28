#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, Ident, LitStr, Token};

/// Generate debug, info, warn and error log macro's
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
        let macro_name = syn::Ident::new(
            &format!("{}_{}", log_type, level),
            proc_macro2::Span::call_site(),
        );

        quote! {
            #[doc = concat!("Writes a ", stringify!(#level), "! message to the ", #log_prefix, "::", #log_type, "logger")]
            #[macro_export]
            macro_rules! #macro_name {
                ($($arg:tt)+) => {
                    log::#level!(target: concat!(#log_prefix, "::", #log_type), $($arg)+)
                };
            }
        }
    });

    quote! {
        #(#macro_exports)*
    }
}
