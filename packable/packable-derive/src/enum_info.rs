// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use syn::{parse_quote, Attribute, DataEnum, Field, Ident, Result, Type};

use crate::{
    parse::filter_attrs, tag_type_info::TagTypeInfo, unpack_error_info::UnpackErrorInfo,
    unpack_visitor_info::UnpackVisitorInfo, variant_info::VariantInfo,
};

pub(crate) struct EnumInfo {
    pub(crate) unpack_error: UnpackErrorInfo,
    pub(crate) unpack_visitor: UnpackVisitorInfo,
    pub(crate) tag_type: TagTypeInfo,
    pub(crate) variants_info: Vec<VariantInfo>,
}

impl EnumInfo {
    pub(crate) fn new(ident: Ident, data: DataEnum, attrs: &[Attribute], crate_name: &Ident) -> Result<Self> {
        let repr_type = attrs
            .iter()
            .find(|attr| attr.path.is_ident("repr"))
            .map(Attribute::parse_args::<Type>)
            .transpose()?;

        let filtered_attrs = filter_attrs(attrs);

        let tag_type = TagTypeInfo::new(&ident, filtered_attrs.clone(), &repr_type, crate_name)?;
        let tag_ty = &tag_type.tag_type;

        let unpack_error = UnpackErrorInfo::new(
            filtered_attrs.clone(),
            || parse_quote!(#crate_name::error::UnknownTagError<#tag_ty>),
        )?;

        let unpack_visitor = UnpackVisitorInfo::new(filtered_attrs, || {
            match data
                .variants
                .iter()
                .next()
                .and_then(|variant| variant.fields.iter().next())
            {
                Some(Field { ty, .. }) => Ok((parse_quote!(<#ty as #crate_name::Packable>::UnpackVisitor), true)),
                None => Ok((parse_quote!(()), false)),
            }
        })?;

        let variants_info = data
            .variants
            .iter()
            .map(|variant| VariantInfo::new(variant, &ident, &unpack_error.with))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            unpack_error,
            unpack_visitor,
            tag_type,
            variants_info,
        })
    }
}
