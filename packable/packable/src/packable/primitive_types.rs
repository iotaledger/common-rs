// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use primitive_types::U256;

use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

impl Packable for U256 {
    type UnpackError = Infallible;
    type UnpackVisitor = ();

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.0.pack(packer)
    }

    #[inline]
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        <[u64; 4]>::unpack::<_, VERIFY>(unpacker, visitor).map(Self)
    }
}
