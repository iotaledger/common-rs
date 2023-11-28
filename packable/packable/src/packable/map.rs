// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and implementations for packing and unpacking map values.

extern crate alloc;

use core::{convert::Infallible, fmt};

/// Error type raised when a semantic error occurs while unpacking a map.
pub enum UnpackMapError<K, V, KE, VE, P> {
    /// A duplicate key.
    OccupiedKey((K, V)),
    /// Semantic error raised while unpacking a key of the map. Typically this is
    /// [`Packable::UnpackError`](crate::Packable::UnpackError).
    Key(KE),
    /// Semantic error raised while unpacking a value of the map. Typically this is
    /// [`Packable::UnpackError`](crate::Packable::UnpackError).
    Value(VE),
    /// Semantic error raised when the length prefix cannot be unpacked.
    Prefix(P),
}

impl<K, V, KE: fmt::Debug, VE: fmt::Debug, P: fmt::Debug> fmt::Debug for UnpackMapError<K, V, KE, VE, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OccupiedKey(_) => f.debug_tuple("OccupiedKey").finish(),
            Self::Key(arg0) => f.debug_tuple("Key").field(arg0).finish(),
            Self::Value(arg0) => f.debug_tuple("Value").field(arg0).finish(),
            Self::Prefix(arg0) => f.debug_tuple("Prefix").field(arg0).finish(),
        }
    }
}

#[cfg(feature = "std")]
impl<K, V, KE, VE, P> std::error::Error for UnpackMapError<K, V, KE, VE, P>
where
    KE: std::error::Error,
    VE: std::error::Error,
    P: std::error::Error,
{
}

impl<K, V, KE, VE, P> From<Infallible> for UnpackMapError<K, V, KE, VE, P> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

impl<K, V, KE: fmt::Display, VE: fmt::Display, P: fmt::Display> fmt::Display for UnpackMapError<K, V, KE, VE, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OccupiedKey(_) => write!(f, "duplicate key in map"),
            Self::Key(err) => write!(f, "cannot unpack key: {}", err),
            Self::Value(err) => write!(f, "cannot unpack value: {}", err),
            Self::Prefix(err) => write!(f, "cannot unpack prefix: {}", err),
        }
    }
}

#[cfg(feature = "usize")]
mod btreemap {
    use alloc::collections::BTreeMap;
    use core::borrow::Borrow;

    use super::*;
    use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

    impl<K: Packable + Ord, V: Packable> Packable for BTreeMap<K, V>
    where
        V::UnpackVisitor: Borrow<K::UnpackVisitor>,
    {
        type UnpackError = UnpackMapError<K, V, K::UnpackError, V::UnpackError, <usize as Packable>::UnpackError>;
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
                // TODO: fails here for the visitor: `expected `&<V as Packable>::UnpackVisitor`, found `&<K as
                // Packable>::UnpackVisitor``
                let value = V::unpack::<_, VERIFY>(unpacker, &visitor)
                    .map_packable_err(UnpackMapError::Value)
                    .map_packable_err(Self::UnpackError::from)?;
                if map.contains_key(&key) {
                    return Err(UnpackError::Packable(Self::UnpackError::OccupiedKey((key, value))));
                } else {
                    map.insert(key, value);
                }
            }

            Ok(map)
        }
    }
}
