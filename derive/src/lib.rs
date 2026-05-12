use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(DeepFlatten)]
pub fn deep_flatten(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl DeepFlatten for #name {
            type Item = #name;
            fn deep_flatten(self) -> Vec<#name> {
                vec![self]
            }
        }
    };

    TokenStream::from(expanded)
}
