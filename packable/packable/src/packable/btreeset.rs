// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::collections::BTreeSet;
use core::{convert::Infallible, fmt};

use crate::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

/// Error type raised when a semantic error occurs while unpacking an option.
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
            Self::Item(arg0) => f.debug_tuple("Item").field(arg0).finish(),
            Self::Prefix(arg0) => f.debug_tuple("Prefix").field(arg0).finish(),
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

#[cfg(feature = "usize")]
impl<T: Packable + Ord> Packable for BTreeSet<T> {
    type UnpackError = UnpackSetError<T, T::UnpackError, <usize as Packable>::UnpackError>;
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
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        use crate::error::UnpackErrorExt;

        let len = u64::unpack::<_, VERIFY>(unpacker, &())
            .coerce()?
            .try_into()
            .map_err(|err| UnpackError::Packable(Self::UnpackError::Prefix(err)))?;

        let mut set = BTreeSet::new();

        for _ in 0..len {
            let item = T::unpack::<_, VERIFY>(unpacker, visitor).map_packable_err(Self::UnpackError::Item)?;
            if set.contains(&item) {
                return Err(UnpackError::Packable(Self::UnpackError::DuplicateItem(item)));
            }
            set.insert(item);
        }

        Ok(set)
    }
}
