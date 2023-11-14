// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use syn::{parse::ParseStream, parse_quote, Attribute, Field, Fields, Ident, Path, Result};

use crate::{
    parse::{filter_attrs, parse_kv, skip_stream},
    record_info::RecordInfo,
    unpack_error_info::UnpackErrorInfo,
    unpack_visitor_info::UnpackVisitorInfo,
};

pub(crate) struct StructInfo {
    pub(crate) unpack_error: UnpackErrorInfo,
    pub(crate) verify_with: Option<Path>,
    pub(crate) unpack_visitor: UnpackVisitorInfo,
    pub(crate) inner: RecordInfo,
}

impl StructInfo {
    pub(crate) fn new(path: Path, fields: &Fields, attrs: &[Attribute], crate_name: &Ident) -> Result<Self> {
        let filtered_attrs = filter_attrs(attrs);

        let unpack_error = UnpackErrorInfo::new(filtered_attrs.clone(), || match fields.iter().next() {
            Some(Field { ty, .. }) => parse_quote!(<#ty as #crate_name::Packable>::UnpackError),
            None => parse_quote!(core::convert::Infallible),
        })?;

        let mut verify_with_opt = None;

        for attr in filtered_attrs.clone() {
            if let Some(verify_with) = attr.parse_args_with(|stream: ParseStream| {
                let opt = parse_kv("verify_with", stream)?;
                if opt.is_none() {
                    skip_stream(stream)?;
                }
                Ok(opt)
            })? {
                verify_with_opt = Some(verify_with);
            }
        }

        let unpack_visitor = UnpackVisitorInfo::new(filtered_attrs, || match fields.iter().next() {
            Some(Field { ty, .. }) => parse_quote!(<#ty as #crate_name::Packable>::UnpackVisitor),
            None => parse_quote!(()),
        })?;

        let inner = RecordInfo::new(path, fields, &unpack_error.with)?;

        Ok(Self {
            unpack_error,
            verify_with: verify_with_opt,
            unpack_visitor,
            inner,
        })
    }
}
