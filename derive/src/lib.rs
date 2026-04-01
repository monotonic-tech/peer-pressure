//! Derive macro~s~ for [peer-pressure](https://docs.rs/peer-pressure).

#![warn(missing_docs)]

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derives an empty `Valid` impl, marking the type as valid by construction.
///
/// ```ignore
/// use peer_pressure::derive::Valid;
///
/// #[derive(Valid)]
/// struct NonEmpty<T>(Vec<T>);
/// ```
#[proc_macro_derive(Valid)]
pub fn derive_valid(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_path = match crate_name("peer-pressure") {
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
        Ok(FoundCrate::Itself) | Err(_) => quote!(::peer_pressure),
    };

    let expanded = quote! {
        impl #impl_generics #crate_path::Valid for #name #ty_generics #where_clause {}
    };

    expanded.into()
}
