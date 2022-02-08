// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::UnpackPrefixError,
    unpacker::Unpacker,
    Packable,
};

use alloc::vec::Vec;

impl<T> Packable for Vec<T>
where
    T: Packable,
{
    type UnpackError = UnpackPrefixError<T::UnpackError, <usize as Packable>::UnpackError>;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // This cast is fine because we know `usize` is not larger than `64` bits.
        (self.len() as u64).pack(packer)?;

        for item in self.iter() {
            item.pack(packer)?;
        }

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let len = u64::unpack::<_, VERIFY>(unpacker)
            .infallible()?
            .try_into()
            .map_err(|err| UnpackError::Packable(UnpackPrefixError::Prefix(err)))?;

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let item = T::unpack::<_, VERIFY>(unpacker).coerce()?;
            vec.push(item);
        }

        Ok(vec)
    }
}
