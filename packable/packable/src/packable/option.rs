// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and utilities related to packing and unpacking [`Option`] values.

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use core::fmt;

/// Error type raised when a semantic error occurs while unpacking an option.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum UnpackOptionError<E> {
    /// The tag found while unpacking is not valid.
    UnknownTag(u8),
    /// A semantic error for the underlying type was raised.
    Inner(E),
}

impl<E> From<E> for UnpackOptionError<E> {
    fn from(err: E) -> Self {
        Self::Inner(err)
    }
}

impl<E: fmt::Display> fmt::Display for UnpackOptionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTag(tag) => write!(f, "unknown tag value {} for option", tag),
            Self::Inner(err) => write!(f, "cannot unpack some variant: {}", err),
        }
    }
}

/// Options are packed and unpacked using `0u8` as the prefix for `None` and `1u8` as the prefix for `Some`.
impl<T: Packable> Packable for Option<T> {
    type UnpackError = UnpackOptionError<T::UnpackError>;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            None => 0u8.pack(packer),
            Some(item) => {
                1u8.pack(packer)?;
                item.pack(packer)
            }
        }
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        match u8::unpack::<_, VERIFY>(unpacker).infallible()? {
            0 => Ok(None),
            1 => Ok(Some(T::unpack::<_, VERIFY>(unpacker).coerce()?)),
            n => Err(UnpackError::Packable(Self::UnpackError::UnknownTag(n))),
        }
    }
}
