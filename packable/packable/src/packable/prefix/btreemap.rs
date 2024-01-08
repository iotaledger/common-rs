// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::collections::BTreeMap;
use core::{
    borrow::Borrow,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use crate::{
    error::UnpackError,
    map::{UnpackMapError, UnpackOrderedMapError},
    packable::bounded::Bounded,
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

/// Wrapper type for `BTreeMap<K, V>` with a length prefix.
/// The set's prefix bounds are provided by `B`, where `B` is a [`Bounded`] type.
/// The prefix type is the `Bounds` type associated with `B`.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct BTreeMapPrefix<K: Ord, V, B: Bounded> {
    inner: BTreeMap<K, V>,
    bounded: PhantomData<B>,
}

impl<K: Ord + fmt::Debug, V: fmt::Debug, B: Bounded> fmt::Debug for BTreeMapPrefix<K, V, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.inner)
        } else {
            write!(f, "{:?}", self.inner)
        }
    }
}

impl<K: Ord, V, B: Bounded> Default for BTreeMapPrefix<K, V, B> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            bounded: PhantomData,
        }
    }
}

impl<K: Ord, V, B: Bounded> Deref for BTreeMapPrefix<K, V, B> {
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// This is fine as slices cannot be resized.
impl<K: Ord, V, B: Bounded> DerefMut for BTreeMapPrefix<K, V, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K: Ord, V, B: Bounded> From<BTreeMapPrefix<K, V, B>> for BTreeMap<K, V> {
    fn from(prefix: BTreeMapPrefix<K, V, B>) -> Self {
        prefix.inner
    }
}

impl<K: Ord, V, B> TryFrom<BTreeMap<K, V>> for BTreeMapPrefix<K, V, B>
where
    B: Bounded,
{
    type Error = <B as TryFrom<usize>>::Error;

    fn try_from(set: BTreeMap<K, V>) -> Result<Self, Self::Error> {
        B::try_from(set.len())?;

        Ok(Self {
            inner: set,
            bounded: PhantomData,
        })
    }
}

impl<K: Ord, V, B> Packable for BTreeMapPrefix<K, V, B>
where
    K: Packable,
    V: Packable,
    B: Bounded + Packable<UnpackVisitor = ()>,
    <B::Bounds as TryInto<B>>::Error: fmt::Debug,
    <B as TryFrom<usize>>::Error: fmt::Debug,
    Range<B::Bounds>: Iterator<Item = B::Bounds>,
    V::UnpackVisitor: Borrow<K::UnpackVisitor>,
{
    type UnpackError = UnpackOrderedMapError<K, K::UnpackError, V::UnpackError, B::UnpackError>;
    type UnpackVisitor = V::UnpackVisitor;

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // The length of any dynamically-sized sequence must be prefixed. This unwrap is fine since
        // the length of the inner slice has been validated while creating this `BTreeMapPrefix`.
        B::try_from(self.len()).unwrap().pack(packer)?;

        for (k, v) in self.iter() {
            k.pack(packer)?;
            v.pack(packer)?;
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
            .map_packable_err(UnpackMapError::Prefix)
            .map_packable_err(Self::UnpackError::from)?
            .into();

        let mut map = BTreeMap::<K, V>::new();

        for _ in B::Bounds::default()..len {
            let key = K::unpack::<_, VERIFY>(unpacker, visitor.borrow())
                .map_packable_err(UnpackMapError::Key)
                .map_packable_err(Self::UnpackError::from)?;
            if let Some((last, _)) = map.last_key_value() {
                match last.cmp(&key) {
                    core::cmp::Ordering::Equal => {
                        return Err(UnpackError::Packable(Self::UnpackError::Map(
                            UnpackMapError::DuplicateKey(key),
                        )));
                    }
                    core::cmp::Ordering::Greater => {
                        return Err(UnpackError::Packable(Self::UnpackError::Unordered));
                    }
                    core::cmp::Ordering::Less => (),
                }
            }
            let value = V::unpack::<_, VERIFY>(unpacker, visitor)
                .map_packable_err(UnpackMapError::Value)
                .map_packable_err(Self::UnpackError::from)?;
            map.insert(key, value);
        }

        Ok(Self {
            inner: map,
            bounded: PhantomData,
        })
    }
}
