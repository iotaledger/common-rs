// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

use alloc::boxed::Box;
#[cfg(feature = "usize")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "usize")]
use core::any::TypeId;
use core::ops::Deref;

impl<T: Packable> Packable for Box<T> {
    type UnpackError = T::UnpackError;

    #[inline(always)]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.deref().pack(packer)
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(Box::new(T::unpack::<_, VERIFY>(unpacker)?))
    }
}

#[cfg(feature = "usize")]
impl<T: Packable> Packable for Box<[T]> {
    type UnpackError = crate::prefix::UnpackPrefixError<T::UnpackError, <usize as Packable>::UnpackError>;

    #[inline(always)]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // This cast is fine because we know `usize` is not larger than `64` bits.
        (self.len() as u64).pack(packer)?;

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            // Safety: `Self` is identical to `Box<[u8]>`.
            let bytes = unsafe { core::mem::transmute::<&Self, &Box<[u8]>>(self) };
            packer.pack_bytes(bytes)?;
        } else {
            for item in self.iter() {
                item.pack(packer)?;
            }
        }

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        use crate::error::UnpackErrorExt;

        let len = u64::unpack::<_, VERIFY>(unpacker)
            .infallible()?
            .try_into()
            .map_err(|err| UnpackError::Packable(Self::UnpackError::Prefix(err)))?;

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let mut bytes = vec![0u8; len].into_boxed_slice();
            unpacker.unpack_bytes(&mut bytes)?;
            // Safety: `Self` is identical to `Box<[u8]>`.
            Ok(unsafe { core::mem::transmute::<Box<[u8]>, Self>(bytes) })
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                let item = T::unpack::<_, VERIFY>(unpacker).coerce()?;
                vec.push(item);
            }

            Ok(vec.into_boxed_slice())
        }
    }
}
