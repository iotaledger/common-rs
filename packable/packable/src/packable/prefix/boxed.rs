// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::{
    any::TypeId,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use crate::{
    error::UnpackError, packable::bounded::Bounded, packer::Packer, prefix::vec::VecPrefix, unpacker::Unpacker,
    Packable,
};

/// Wrapper type for `Box<[T]>` with a length prefix.
/// The boxed slice's prefix bounds are provided by `B`, where `B` is a [`Bounded`] type.
/// The prefix type is the `Bounds` type associated with `B`.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct BoxedSlicePrefix<T, B: Bounded> {
    inner: Box<[T]>,
    bounded: PhantomData<B>,
}

impl<T: fmt::Debug, B: Bounded> fmt::Debug for BoxedSlicePrefix<T, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "BoxedSlicePrefix({:#?})", self.inner)
        } else {
            write!(f, "BoxedSlicePrefix({:?})", self.inner)
        }
    }
}

impl<T, B: Bounded> Default for BoxedSlicePrefix<T, B> {
    fn default() -> Self {
        Self {
            inner: Box::new([]),
            bounded: PhantomData,
        }
    }
}

impl<T, B: Bounded> Deref for BoxedSlicePrefix<T, B> {
    type Target = Box<[T]>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// This is fine as slices cannot be resized.
impl<T, B: Bounded> DerefMut for BoxedSlicePrefix<T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, B: Bounded> From<BoxedSlicePrefix<T, B>> for Box<[T]> {
    fn from(prefix: BoxedSlicePrefix<T, B>) -> Self {
        prefix.inner
    }
}

impl<T, B> TryFrom<Box<[T]>> for BoxedSlicePrefix<T, B>
where
    B: Bounded,
{
    type Error = <B as TryFrom<usize>>::Error;

    fn try_from(boxed_slice: Box<[T]>) -> Result<Self, Self::Error> {
        B::try_from(boxed_slice.len())?;

        Ok(Self {
            inner: boxed_slice,
            bounded: PhantomData,
        })
    }
}

impl<T, B> Packable for BoxedSlicePrefix<T, B>
where
    T: Packable,
    B: Bounded + Packable,
    <B::Bounds as TryInto<B>>::Error: fmt::Debug,
    <B as TryFrom<usize>>::Error: fmt::Debug,
    Range<B::Bounds>: Iterator<Item = B::Bounds>,
{
    type UnpackError = <VecPrefix<T, B> as Packable>::UnpackError;

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // The length of any dynamically-sized sequence must be prefixed. This unwrap is fine since
        // the length of the inner slice has been validated while creating this `BoxedSlicePrefix`.
        B::try_from(self.len()).unwrap().pack(packer)?;

        if TypeId::of::<T>() == TypeId::of::<u8>() {
            // Safety: `Self` is identical to `BoxedSlicePrefix<u8, B>`.
            let bytes = unsafe { core::mem::transmute::<&Self, &BoxedSlicePrefix<u8, B>>(self) };
            packer.pack_bytes(bytes.deref())?;
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
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let vec: Vec<T> = VecPrefix::<T, B>::unpack::<_, VERIFY>(unpacker)?.into();

        Ok(Self {
            inner: vec.into_boxed_slice(),
            bounded: PhantomData,
        })
    }
}
