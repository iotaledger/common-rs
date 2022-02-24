// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

use core::convert::Infallible;

macro_rules! impl_packable_for_num {
    ($ty:ty) => {
        impl Packable for $ty {
            type UnpackError = Infallible;

            #[inline(always)]
            fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
                packer.pack_bytes(&self.to_le_bytes())
            }

            fn unpack<U: Unpacker, const VERIFY: bool>(
                unpacker: &mut U,
            ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
                let mut bytes = [0u8; core::mem::size_of::<Self>()];
                unpacker.unpack_bytes(&mut bytes)?;
                Ok(Self::from_le_bytes(bytes))
            }
        }
    };
}

impl_packable_for_num!(u8);
impl_packable_for_num!(u16);
impl_packable_for_num!(u32);
impl_packable_for_num!(u64);
#[cfg(has_u128)]
impl_packable_for_num!(u128);

impl_packable_for_num!(i8);
impl_packable_for_num!(i16);
impl_packable_for_num!(i32);
impl_packable_for_num!(i64);
#[cfg(has_i128)]
impl_packable_for_num!(i128);

impl_packable_for_num!(f32);
impl_packable_for_num!(f64);

#[cfg(feature = "usize")]
impl Packable for usize {
    type UnpackError = core::num::TryFromIntError;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        const _: () = {
            if core::mem::size_of::<usize>() > core::mem::size_of::<u64>() {
                panic!("The \"usize\" feature cannot be used for targets with a pointer size larger than 64 bits.");
            }
        };

        (*self as u64).pack(packer)
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        use crate::error::UnpackErrorExt;
        Self::try_from(u64::unpack::<_, VERIFY>(unpacker).coerce()?).map_err(UnpackError::Packable)
    }
}

#[cfg(feature = "usize")]
impl Packable for isize {
    type UnpackError = core::num::TryFromIntError;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        (*self as i64).pack(packer)
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        use crate::error::UnpackErrorExt;
        Self::try_from(i64::unpack::<_, VERIFY>(unpacker).coerce()?).map_err(UnpackError::Packable)
    }
}
