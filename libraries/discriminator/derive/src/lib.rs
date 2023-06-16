//! Derive macro library for the `spl-discriminator` library

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use spl_discriminator_syn::SplDiscriminatorBuilder;
use syn::parse_macro_input;

/// Derive macro library to implement the `SplDiscriminator` trait
/// on an enum or struct
#[proc_macro_derive(SplDiscriminator, attributes(discriminator_namespace))]
pub fn spl_discriminator(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as SplDiscriminatorBuilder)
        .to_token_stream()
        .into()
}
