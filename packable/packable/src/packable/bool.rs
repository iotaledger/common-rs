// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

impl Packable for bool {
    type UnpackError = Infallible;
    type UnpackVisitor = ();

    /// Booleans are packed as `u8` integers following Rust's data layout.
    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        (*self as u8).pack(packer)
    }

    /// Booleans are unpacked if the byte used to represent them is non-zero.
    #[inline]
    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(u8::unpack(unpacker, visitor).coerce()? != 0)
    }
}
