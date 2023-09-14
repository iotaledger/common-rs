// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::{vec, vec::Vec};
use core::any::TypeId;

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::UnpackPrefixError,
    unpacker::Unpacker,
    Packable,
};

impl<T> Packable for Vec<T>
where
    T: Packable,
{
    type UnpackError = UnpackPrefixError<T::UnpackError, <usize as Packable>::UnpackError>;
    type UnpackVisitor = T::UnpackVisitor;

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // This cast is fine because we know `usize` is not larger than `64` bits.
        (self.len() as u64).pack(packer)?;

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            // Safety: `Self` is identical to `Vec<u8>`.
            let bytes = unsafe { core::mem::transmute::<&Self, &Vec<u8>>(self) };
            packer.pack_bytes(bytes)?;
        } else {
            for item in self.iter() {
                item.pack(packer)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let len = u64::unpack::<_, VERIFY>(unpacker, &())
            .coerce()?
            .try_into()
            .map_err(|err| UnpackError::Packable(UnpackPrefixError::Prefix(err)))?;

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let mut bytes = vec![0u8; len];
            unpacker.unpack_bytes(&mut bytes)?;
            // Safety: `Self` is identical to `Vec<u8>`.
            Ok(unsafe { core::mem::transmute::<Vec<u8>, Self>(bytes) })
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                let item = T::unpack::<_, VERIFY>(unpacker, visitor).map_packable_err(Self::UnpackError::Item)?;
                vec.push(item);
            }

            Ok(vec)
        }
    }
}
