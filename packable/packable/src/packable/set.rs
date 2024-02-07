// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and implementations for packing and unpacking set values.

extern crate alloc;

use core::{convert::Infallible, fmt};

/// Error type raised when a semantic error occurs while unpacking a set.
pub enum UnpackSetError<T, I, P> {
    /// A duplicate set item.
    DuplicateItem(T),
    /// Semantic error raised while unpacking an item of the sequence. Typically this is
    /// [`Packable::UnpackError`](crate::Packable::UnpackError).
    Item(I),
    /// Semantic error raised when the length prefix cannot be unpacked.
    Prefix(P),
}

impl<T, I: fmt::Debug, P: fmt::Debug> fmt::Debug for UnpackSetError<T, I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateItem(_) => f.debug_tuple("DuplicateItem").finish(),
            Self::Item(err) => f.debug_tuple("Item").field(err).finish(),
            Self::Prefix(err) => f.debug_tuple("Prefix").field(err).finish(),
        }
    }
}

#[cfg(feature = "std")]
impl<T, I, P> std::error::Error for UnpackSetError<T, I, P>
where
    I: std::error::Error,
    P: std::error::Error,
{
}

impl<T, I, P> From<Infallible> for UnpackSetError<T, I, P> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

impl<T, I: fmt::Display, P: fmt::Display> fmt::Display for UnpackSetError<T, I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateItem(_) => write!(f, "duplicate item in set"),
            Self::Item(err) => write!(f, "cannot unpack item: {}", err),
            Self::Prefix(err) => write!(f, "cannot unpack prefix: {}", err),
        }
    }
}

/// Error type raised when a semantic error occurs while unpacking an ordered set.
pub enum UnpackOrderedSetError<T, I, P> {
    /// A set error.
    Set(UnpackSetError<T, I, P>),
    /// An unordered set.
    Unordered,
}

impl<T, I, P> From<UnpackSetError<T, I, P>> for UnpackOrderedSetError<T, I, P> {
    fn from(value: UnpackSetError<T, I, P>) -> Self {
        Self::Set(value)
    }
}

impl<T, I: fmt::Debug, P: fmt::Debug> fmt::Debug for UnpackOrderedSetError<T, I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Set(err) => f.debug_tuple("Set").field(err).finish(),
            Self::Unordered => f.debug_tuple("Unordered").finish(),
        }
    }
}

#[cfg(feature = "std")]
impl<T, I, P> std::error::Error for UnpackOrderedSetError<T, I, P>
where
    I: std::error::Error,
    P: std::error::Error,
{
}

impl<T, I, P> From<Infallible> for UnpackOrderedSetError<T, I, P> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

impl<T, I: fmt::Display, P: fmt::Display> fmt::Display for UnpackOrderedSetError<T, I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Set(err) => err.fmt(f),
            Self::Unordered => write!(f, "unordered set"),
        }
    }
}

#[cfg(feature = "usize")]
mod btreeset {
    use alloc::collections::BTreeSet;

    use super::*;
    use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

    impl<T: Packable + Ord> Packable for BTreeSet<T> {
        type UnpackError = UnpackOrderedSetError<T, T::UnpackError, <usize as Packable>::UnpackError>;
        type UnpackVisitor = T::UnpackVisitor;

        #[inline]
        fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
            // This cast is fine because we know `usize` is not larger than `64` bits.
            (self.len() as u64).pack(packer)?;

            for item in self.iter() {
                item.pack(packer)?;
            }

            Ok(())
        }

        #[inline]
        fn unpack<U: Unpacker>(
            unpacker: &mut U,
            visitor: Option<&Self::UnpackVisitor>,
        ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
            use crate::error::UnpackErrorExt;

            let len = u64::unpack(unpacker, None)
                .coerce()?
                .try_into()
                .map_err(|err| UnpackError::Packable(UnpackSetError::Prefix(err).into()))?;

            let mut set = BTreeSet::<T>::new();

            for _ in 0..len {
                let item = T::unpack(unpacker, visitor)
                    .map_packable_err(UnpackSetError::Item)
                    .map_packable_err(Self::UnpackError::from)?;

                if let Some(last) = set.last() {
                    match last.cmp(&item) {
                        core::cmp::Ordering::Equal => {
                            return Err(UnpackError::Packable(Self::UnpackError::Set(
                                UnpackSetError::DuplicateItem(item),
                            )));
                        }
                        core::cmp::Ordering::Greater => {
                            return Err(UnpackError::Packable(Self::UnpackError::Unordered));
                        }
                        core::cmp::Ordering::Less => (),
                    }
                }

                set.insert(item);
            }

            Ok(set)
        }
    }
}
