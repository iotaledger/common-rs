// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::collections::BTreeSet;
use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use crate::{
    error::UnpackError,
    packable::{bounded::Bounded, set::UnpackSetError},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

/// Wrapper type for `BTreeSet<T>` with a length prefix.
/// The set's prefix bounds are provided by `B`, where `B` is a [`Bounded`] type.
/// The prefix type is the `Bounds` type associated with `B`.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct BTreeSetPrefix<T: Ord, B: Bounded> {
    inner: BTreeSet<T>,
    bounded: PhantomData<B>,
}

impl<T: Ord + fmt::Debug, B: Bounded> fmt::Debug for BTreeSetPrefix<T, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.inner)
        } else {
            write!(f, "{:?}", self.inner)
        }
    }
}

impl<T: Ord, B: Bounded> Default for BTreeSetPrefix<T, B> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            bounded: PhantomData,
        }
    }
}

impl<T: Ord, B: Bounded> Deref for BTreeSetPrefix<T, B> {
    type Target = BTreeSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// This is fine as slices cannot be resized.
impl<T: Ord, B: Bounded> DerefMut for BTreeSetPrefix<T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Ord, B: Bounded> From<BTreeSetPrefix<T, B>> for BTreeSet<T> {
    fn from(prefix: BTreeSetPrefix<T, B>) -> Self {
        prefix.inner
    }
}

impl<T: Ord, B> TryFrom<BTreeSet<T>> for BTreeSetPrefix<T, B>
where
    B: Bounded,
{
    type Error = <B as TryFrom<usize>>::Error;

    fn try_from(set: BTreeSet<T>) -> Result<Self, Self::Error> {
        B::try_from(set.len())?;

        Ok(Self {
            inner: set,
            bounded: PhantomData,
        })
    }
}

impl<T: Ord, B> Packable for BTreeSetPrefix<T, B>
where
    T: Packable,
    B: Bounded + Packable<UnpackVisitor = ()>,
    <B::Bounds as TryInto<B>>::Error: fmt::Debug,
    <B as TryFrom<usize>>::Error: fmt::Debug,
    Range<B::Bounds>: Iterator<Item = B::Bounds>,
{
    type UnpackError = UnpackSetError<T, T::UnpackError, B::UnpackError>;
    type UnpackVisitor = T::UnpackVisitor;

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // The length of any dynamically-sized sequence must be prefixed. This unwrap is fine since
        // the length of the inner slice has been validated while creating this `BTreeSetPrefix`.
        B::try_from(self.len()).unwrap().pack(packer)?;

        for item in self.iter() {
            item.pack(packer)?;
        }

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        use crate::error::UnpackErrorExt;

        // The length of any dynamically-sized sequence must be prefixed.
        let len = B::unpack::<_, VERIFY>(unpacker, &())
            .map_packable_err(Self::UnpackError::Prefix)?
            .into();

        let mut set = BTreeSet::new();

        for _ in B::Bounds::default()..len {
            let item = T::unpack::<_, VERIFY>(unpacker, visitor).map_packable_err(Self::UnpackError::Item)?;
            if set.contains(&item) {
                return Err(UnpackError::Packable(Self::UnpackError::DuplicateItem(item)));
            }
            set.insert(item);
        }

        Ok(Self {
            inner: set,
            bounded: PhantomData,
        })
    }
}
