use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

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