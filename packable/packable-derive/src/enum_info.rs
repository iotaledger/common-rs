// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use syn::{parse_quote, Attribute, DataEnum, Ident, Result, Type};

use crate::{
    parse::filter_attrs, tag_type_info::TagTypeInfo, unpack_error_info::UnpackErrorInfo, variant_info::VariantInfo,
};

pub(crate) struct EnumInfo {
    pub(crate) unpack_error: UnpackErrorInfo,
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
            filtered_attrs,
            || parse_quote!(#crate_name::error::UnknownTagError<#tag_ty>),
        )?;

        let variants_info = data
            .variants
            .iter()
            .map(|variant| VariantInfo::new(variant, &ident, &unpack_error.with))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            unpack_error,
            tag_type,
            variants_info,
        })
    }
}
