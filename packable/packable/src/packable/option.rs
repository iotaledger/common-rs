// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and utilities related to packing and unpacking [`Option`] values.

use core::{convert::Infallible, fmt};

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

/// Error type raised when a semantic error occurs while unpacking an option.
#[derive(Debug)]
pub enum UnpackOptionError<E> {
    /// The tag found while unpacking is not valid.
    UnknownTag(u8),
    /// A semantic error for the underlying type was raised.
    Inner(E),
}

#[cfg(feature = "std")]
impl<E> std::error::Error for UnpackOptionError<E> where E: std::error::Error {}

impl<E> From<Infallible> for UnpackOptionError<E> {
    fn from(err: Infallible) -> Self {
        match err {}
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
    type UnpackVisitor = T::UnpackVisitor;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            None => 0u8.pack(packer),
            Some(item) => {
                1u8.pack(packer)?;
                item.pack(packer)
            }
        }
    }

    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        match u8::unpack_inner(unpacker, visitor).coerce()? {
            0 => Ok(None),
            1 => Ok(Some(
                T::unpack(unpacker, visitor).map_packable_err(UnpackOptionError::Inner)?,
            )),
            n => Err(UnpackError::Packable(Self::UnpackError::UnknownTag(n))),
        }
    }
}
