// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and implementations for packing and unpacking map values.

extern crate alloc;

#[cfg(feature = "usize")]
use core::borrow::Borrow;
use core::{convert::Infallible, fmt};
#[cfg(feature = "std")]
use std::{collections::HashMap, hash::Hash};

#[cfg(feature = "usize")]
use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

/// Error type raised when a semantic error occurs while unpacking a map.
pub enum UnpackMapError<K, KE, VE, P> {
    /// A duplicate key.
    DuplicateKey(K),
    /// Semantic error raised while unpacking a key of the map. Typically this is
    /// [`Packable::UnpackError`].
    Key(KE),
    /// Semantic error raised while unpacking a value of the map. Typically this is
    /// [`Packable::UnpackError`].
    Value(VE),
    /// Semantic error raised when the length prefix cannot be unpacked.
    Prefix(P),
}

impl<K, KE: fmt::Debug, VE: fmt::Debug, P: fmt::Debug> fmt::Debug for UnpackMapError<K, KE, VE, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateKey(_) => f.debug_tuple("DuplicateKey").finish(),
            Self::Key(err) => f.debug_tuple("Key").field(err).finish(),
            Self::Value(err) => f.debug_tuple("Value").field(err).finish(),
            Self::Prefix(err) => f.debug_tuple("Prefix").field(err).finish(),
        }
    }
}

#[cfg(feature = "std")]
impl<K, KE, VE, P> std::error::Error for UnpackMapError<K, KE, VE, P>
where
    KE: std::error::Error,
    VE: std::error::Error,
    P: std::error::Error,
{
}

impl<K, KE, VE, P> From<Infallible> for UnpackMapError<K, KE, VE, P> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

impl<K, KE: fmt::Display, VE: fmt::Display, P: fmt::Display> fmt::Display for UnpackMapError<K, KE, VE, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateKey(_) => write!(f, "duplicate key in map"),
            Self::Key(err) => write!(f, "cannot unpack key: {}", err),
            Self::Value(err) => write!(f, "cannot unpack value: {}", err),
            Self::Prefix(err) => write!(f, "cannot unpack prefix: {}", err),
        }
    }
}

/// Error type raised when a semantic error occurs while unpacking an ordered map.
pub enum UnpackOrderedMapError<K, KE, VE, P> {
    /// A map error.
    Map(UnpackMapError<K, KE, VE, P>),
    /// An unordered map.
    Unordered,
}

impl<K, KE, VE, P> From<UnpackMapError<K, KE, VE, P>> for UnpackOrderedMapError<K, KE, VE, P> {
    fn from(value: UnpackMapError<K, KE, VE, P>) -> Self {
        Self::Map(value)
    }
}

impl<K, KE: fmt::Debug, VE: fmt::Debug, P: fmt::Debug> fmt::Debug for UnpackOrderedMapError<K, KE, VE, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Map(err) => f.debug_tuple("Map").field(err).finish(),
            Self::Unordered => f.debug_tuple("Unordered").finish(),
        }
    }
}

#[cfg(feature = "std")]
impl<K, KE, VE, P> std::error::Error for UnpackOrderedMapError<K, KE, VE, P>
where
    KE: std::error::Error,
    VE: std::error::Error,
    P: std::error::Error,
{
}

impl<K, KE, VE, P> From<Infallible> for UnpackOrderedMapError<K, KE, VE, P> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

impl<K, KE: fmt::Display, VE: fmt::Display, P: fmt::Display> fmt::Display for UnpackOrderedMapError<K, KE, VE, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Map(s) => s.fmt(f),
            Self::Unordered => write!(f, "unordered map"),
        }
    }
}

#[cfg(all(feature = "std", feature = "usize"))]
impl<K: Packable + Ord + Hash, V: Packable> Packable for HashMap<K, V>
where
    V::UnpackVisitor: Borrow<K::UnpackVisitor>,
{
    type UnpackError = UnpackMapError<K, K::UnpackError, V::UnpackError, <usize as Packable>::UnpackError>;
    type UnpackVisitor = V::UnpackVisitor;

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // This cast is fine because we know `usize` is not larger than `64` bits.
        (self.len() as u64).pack(packer)?;

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

        let len = u64::unpack::<_, VERIFY>(unpacker, &())
            .coerce()?
            .try_into()
            .map_err(|err| UnpackError::Packable(UnpackMapError::Prefix(err)))?;

        let mut map = HashMap::<K, V>::new();

        for _ in 0..len {
            let key = K::unpack::<_, VERIFY>(unpacker, visitor.borrow())
                .map_packable_err(UnpackMapError::Key)
                .map_packable_err(Self::UnpackError::from)?;

            if map.contains_key(&key) {
                return Err(UnpackError::Packable(UnpackMapError::DuplicateKey(key)));
            }

            let value = V::unpack::<_, VERIFY>(unpacker, visitor)
                .map_packable_err(UnpackMapError::Value)
                .map_packable_err(Self::UnpackError::from)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

#[cfg(feature = "usize")]
mod btreemap {
    use alloc::collections::BTreeMap;

    use super::*;

    impl<K: Packable + Ord, V: Packable> Packable for BTreeMap<K, V>
    where
        V::UnpackVisitor: Borrow<K::UnpackVisitor>,
    {
        type UnpackError = UnpackOrderedMapError<K, K::UnpackError, V::UnpackError, <usize as Packable>::UnpackError>;
        type UnpackVisitor = V::UnpackVisitor;

        #[inline]
        fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
            // This cast is fine because we know `usize` is not larger than `64` bits.
            (self.len() as u64).pack(packer)?;

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

            let len = u64::unpack::<_, VERIFY>(unpacker, &())
                .coerce()?
                .try_into()
                .map_err(|err| UnpackError::Packable(UnpackMapError::Prefix(err).into()))?;

            let mut map = BTreeMap::<K, V>::new();

            for _ in 0..len {
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

            Ok(map)
        }
    }
}
