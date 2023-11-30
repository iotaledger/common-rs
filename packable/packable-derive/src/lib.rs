// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This crate provides the `Packable` derive macro.

mod enum_info;
mod field_info;
mod fragments;
mod parse;
mod record_info;
mod struct_info;
mod tag_type_info;
mod trait_impl;
mod unpack_error_info;
mod unpack_visitor_info;
mod variant_info;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::ToTokens;
use syn::{parse_macro_input, Ident};

use self::trait_impl::TraitImpl;

#[proc_macro_derive(Packable, attributes(packable))]
pub fn packable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let crate_string = match crate_name("packable").expect("packable should be present in `Cargo.toml`") {
        FoundCrate::Itself => "packable_crate".to_owned(),
        FoundCrate::Name(name) => name,
    };

    match TraitImpl::new(input, Ident::new(&crate_string, Span::call_site())) {
        Ok(trait_impl) => trait_impl.into_token_stream(),
        Err(err) => err.into_compile_error(),
    }
    .into()
}
