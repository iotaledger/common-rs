// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error, Result,
};

use crate::parse::{parse_kv, skip_stream};

pub(crate) struct UnpackVisitorInfo {
    pub(crate) unpack_visitor: syn::Type,
}

struct Type(syn::Type);

impl Parse for Type {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::Type::parse(input).map(Self).map_err(|err| {
            Error::new(
                err.span(),
                "The `unpack_visitor` attribute requires a type for its value.",
            )
        })
    }
}

impl UnpackVisitorInfo {
    pub(crate) fn new<'a>(
        filtered_attrs: impl Iterator<Item = &'a Attribute>,
        default_unpack_visitor: impl FnOnce() -> syn::Type,
    ) -> Result<Self> {
        for attr in filtered_attrs {
            let opt_info =
                attr.parse_args_with(
                    |stream: ParseStream| match parse_kv::<Type>("unpack_visitor", stream)? {
                        Some(Type(unpack_visitor)) => Ok(Some(Self { unpack_visitor })),
                        None => {
                            skip_stream(stream)?;
                            Ok(None)
                        }
                    },
                )?;

            if let Some(info) = opt_info {
                return Ok(info);
            }
        }

        Ok(Self {
            unpack_visitor: default_unpack_visitor(),
        })
    }
}
