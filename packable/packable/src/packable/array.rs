// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

use core::{any::TypeId, mem::MaybeUninit};

impl<T: Packable, const N: usize> Packable for [T; N] {
    type UnpackError = T::UnpackError;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            // Safety: `Self` is identical to `[u8; N]`.
            let bytes = unsafe { core::mem::transmute::<&Self, &[u8; N]>(self) };
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
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let mut bytes = [0u8; N];
            unpacker.unpack_bytes(&mut bytes)?;
            // Safety: `Self` is identical to `[u8; N]`.
            Ok(unsafe { (&bytes as *const [u8; N] as *const Self).read() })
        } else {
            // Safety: an uninitialized array of [`MaybeUninit`]s is safe to be considered initialized.
            // FIXME: replace with [`MaybeUninit::uninit_array`] when stabilized.
            let mut array = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };

            for item in array.iter_mut() {
                let unpacked = T::unpack::<_, VERIFY>(unpacker)?;

                // Safety: each `item` is only visited once so we are never overwriting nor dropping values that are
                // already initialized.
                unsafe {
                    item.as_mut_ptr().write(unpacked);
                }
            }

            // Safety: We traversed the whole array and initialized every item.
            // FIXME: replace with [`MaybeUninit::array_assume_init`] when stabilized.
            Ok(unsafe { (&array as *const [MaybeUninit<T>; N] as *const Self).read() })
        }
    }
}
