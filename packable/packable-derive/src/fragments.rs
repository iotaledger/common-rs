// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::{record_info::RecordInfo, unpack_visitor_info::UnpackVisitorInfo};

pub(crate) struct Fragments {
    // The pattern used to destructure the record.
    pub(crate) pattern: TokenStream,
    // An expression that packs the record.
    pub(crate) pack: TokenStream,
    // An expression that unpacks the record.
    pub(crate) unpack: TokenStream,
}

impl Fragments {
    pub(crate) fn new(
        info: RecordInfo,
        verify_with: Option<Path>,
        unpack_visitor_info: &UnpackVisitorInfo,
        crate_name: &Ident,
    ) -> Self {
        let RecordInfo {
            path,
            fields_unpack_error_with,
            fields_verify_with,
            fields_ident,
            fields_pattern_ident,
            fields_type,
        } = info;

        let fields_verification = fields_verify_with.into_iter().zip(fields_ident.iter()).map(
            |(verify_with, field_ident)| match verify_with {
                Some(verify_with) => if unpack_visitor_info.explicit {
                    quote! {
                        if let Some(visitor) = visitor {
                            #verify_with(&#field_ident, visitor).map_err(#crate_name::error::UnpackError::from_packable)?;
                        }
                    }
                } else {
                    quote! {
                        if visitor.is_some() {
                            #verify_with(&#field_ident).map_err(#crate_name::error::UnpackError::from_packable)?;
                        }
                    }
                },
                None => quote!(),
            },
        );

        let verify_with = match verify_with {
            Some(verify_with) => {
                if unpack_visitor_info.explicit {
                    quote! {
                        if let Some(visitor) = visitor {
                            #verify_with(&unpacked, visitor).map_err(#crate_name::error::UnpackError::from_packable)?;
                        }
                    }
                } else {
                    quote! {
                        if visitor.is_some() {
                            #verify_with(&unpacked).map_err(#crate_name::error::UnpackError::from_packable)?;
                        }
                    }
                }
            }
            None => quote!(),
        };

        Self {
            pattern: quote!(#path { #(#fields_pattern_ident: #fields_ident),* }),
            pack: quote! {
                #(<#fields_type as #crate_name::Packable>::pack(#fields_ident, packer)?;) *
                Ok(())
            },
            unpack: quote! {
                #(
                    let #fields_ident = <#fields_type as #crate_name::Packable>::unpack(unpacker, visitor.map(Borrow::<<#fields_type as #crate_name::Packable>::UnpackVisitor>::borrow)).map_packable_err(#fields_unpack_error_with).coerce()?;
                    #fields_verification
                )*

                let unpacked = #path {
                    #(#fields_pattern_ident: #fields_ident,)*
                };

                #verify_with

                Ok(unpacked)
            },
        }
    }
}
