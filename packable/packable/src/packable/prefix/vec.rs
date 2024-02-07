// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::vec::Vec;
use core::{
    any::TypeId,
    fmt,
    marker::PhantomData,
    ops::{Deref, Range},
};

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packable::bounded::Bounded,
    packer::Packer,
    prefix::UnpackPrefixError,
    unpacker::Unpacker,
    Packable,
};

/// Wrapper type for [`Vec<T>`] with a length prefix.
/// The [`Vec<T>`]'s prefix bounds are provided by `B`, where `B` is a [`Bounded`] type.
/// The prefix type is the `Bounds` type associated with `B`.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct VecPrefix<T, B: Bounded> {
    inner: Vec<T>,
    bounded: PhantomData<B>,
}

impl<T: fmt::Debug, B: Bounded> fmt::Debug for VecPrefix<T, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.inner)
        } else {
            write!(f, "{:?}", self.inner)
        }
    }
}

impl<T, B: Bounded> Default for VecPrefix<T, B> {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            bounded: PhantomData,
        }
    }
}

impl<T, B: Bounded> Deref for VecPrefix<T, B> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, B: Bounded> From<VecPrefix<T, B>> for Vec<T> {
    fn from(prefix: VecPrefix<T, B>) -> Self {
        prefix.inner
    }
}

impl<T, B> TryFrom<Vec<T>> for VecPrefix<T, B>
where
    B: Bounded,
{
    type Error = <B as TryFrom<usize>>::Error;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        B::try_from(vec.len())?;

        Ok(Self {
            inner: vec,
            bounded: PhantomData,
        })
    }
}

impl<T, B> Packable for VecPrefix<T, B>
where
    T: Packable,
    B: Bounded + Packable<UnpackVisitor = ()>,
    <B::Bounds as TryInto<B>>::Error: fmt::Debug,
    <B as TryFrom<usize>>::Error: fmt::Debug,
    Range<B::Bounds>: Iterator<Item = B::Bounds>,
{
    type UnpackError = UnpackPrefixError<T::UnpackError, B::UnpackError>;
    type UnpackVisitor = T::UnpackVisitor;

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // The length of any dynamically-sized sequence must be prefixed. This unwrap is fine since
        // the length of the inner `Vec` has been validated while creating this `VecPrefix`.
        B::try_from(self.len()).unwrap().pack(packer)?;
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            // Safety: `Self` is identical to `VecPrefix<u8, B>`.
            let bytes = unsafe { core::mem::transmute::<&Self, &VecPrefix<u8, B>>(self) };
            packer.pack_bytes(bytes.deref())?;
        } else {
            for item in self.iter() {
                item.pack(packer)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        // The length of any dynamically-sized sequence must be prefixed.
        let len = B::unpack(unpacker, None)
            .map_packable_err(UnpackPrefixError::Prefix)?
            .into();

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            // If `len` does not fit in a `usize`, we panic. There is no way this sequence will fit in memory anyway.
            let len = len
                .try_into()
                .ok()
                .expect("the length prefix exceeds the pointer length of this platform");

            let mut bytes = alloc::vec![0u8; len];
            unpacker.unpack_bytes(&mut bytes)?;
            // Safety: `Self` is identical to `VecPrefix<u8, B>` which has the same layout as
            // `Vec<u8>` thanks to `#[repr(transparent)]`.
            Ok(unsafe { core::mem::transmute::<Vec<u8>, Self>(bytes) })
        } else {
            // If `len` fits in a `usize`, we use it as the capacity of the inner `Vec` to avoid extra
            // allocations.
            //
            // If that is not the case, we avoid assuming anything about the memory capacity of the
            // current platform and initialize `inner` with capacity zero. Most of the time this will
            // cause the program to panic due to memory exhaustion or capacity overflow while calling
            // `inner.push` but that is a platform limitation and not an error that the `Packable`
            // infrastructure should handle.
            let mut inner = Vec::with_capacity(len.try_into().unwrap_or(0));

            for _ in B::Bounds::default()..len {
                let item = T::unpack(unpacker, visitor).map_packable_err(Self::UnpackError::Item)?;
                inner.push(item);
            }

            Ok(VecPrefix {
                inner,
                bounded: PhantomData,
            })
        }
    }
}
