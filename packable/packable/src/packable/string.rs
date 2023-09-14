// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::{
    string::{FromUtf8Error, String},
    vec::Vec,
};

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::UnpackPrefixError,
    unpacker::Unpacker,
    Packable,
};

impl Packable for String {
    type UnpackError = UnpackPrefixError<FromUtf8Error, <usize as Packable>::UnpackError>;
    type UnpackVisitor = ();

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        let bytes = self.as_bytes();
        // This cast is fine because we know `usize` is not larger than `64` bits.
        (bytes.len() as u64).pack(packer)?;

        packer.pack_bytes(bytes)?;

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let bytes = Vec::<u8>::unpack::<_, VERIFY>(unpacker, visitor)
            .map_packable_err(|err| UnpackPrefixError::Prefix(err.into_prefix_err()))?;

        String::from_utf8(bytes).map_err(|e| UnpackError::Packable(Self::UnpackError::Item(e)))
    }
}
