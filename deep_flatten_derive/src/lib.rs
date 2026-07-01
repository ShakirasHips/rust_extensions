use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DeepFlatten)]
pub fn deep_flatten(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics DeepFlatten for #name #ty_generics #where_clause {
            type Item = #name #ty_generics;
            fn deep_flatten(self) -> impl Iterator<Item = #name #ty_generics> {
                std::iter::once(self)
            }
        }
    };

    TokenStream::from(expanded)
}
