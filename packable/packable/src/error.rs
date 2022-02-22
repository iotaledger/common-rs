// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors related to packable operations.

use core::{convert::Infallible, fmt};

mod sealed {
    use crate::error::UnpackError;

    pub trait Sealed {}

    impl<T, U, V> Sealed for Result<T, UnpackError<U, V>> {}
}

/// Trait providing utility methods for [`Result`] values that use [`UnpackError`] as the `Err` variant.
///
/// The main disadvantage of using `Result<_, UnpackError<_, _>>` is that error coercion must be done explicitly.
/// This trait attempts to ease these conversions.
///
/// This trait is sealed and cannot be implemented by any other type.
pub trait UnpackErrorExt<T, U, V>: sealed::Sealed + Sized {
    /// Maps the [`Packable`](UnpackError::Packable) variant if the result is an error.
    fn map_packable_err<W>(self, f: impl Fn(U) -> W) -> Result<T, UnpackError<W, V>>;

    /// Coerces the [`Packable`](UnpackError::Packable) variant value using [`Into`].
    fn coerce<W>(self) -> Result<T, UnpackError<W, V>>
    where
        U: Into<W>,
    {
        self.map_packable_err(U::into)
    }
}

impl<T, U, V> UnpackErrorExt<T, U, V> for Result<T, UnpackError<U, V>> {
    fn map_packable_err<W>(self, f: impl Fn(U) -> W) -> Result<T, UnpackError<W, V>> {
        self.map_err(|err| match err {
            UnpackError::Packable(err) => UnpackError::Packable(f(err)),
            UnpackError::Unpacker(err) => UnpackError::Unpacker(err),
        })
    }
}

/// Error type raised when [`Packable::unpack`](crate::Packable::unpack) fails.
///
/// If you need to do error coercion use [`UnpackErrorExt`].
#[derive(Debug)]
pub enum UnpackError<T, U> {
    /// Semantic error. Typically this is [`Packable::UnpackError`](crate::Packable::UnpackError).
    Packable(T),
    /// Error produced by the unpacker. Typically this is [`Unpacker::Error`](crate::unpacker::Unpacker::Error).
    Unpacker(U),
}

impl<T, U> UnpackError<T, U> {
    /// Wraps an error in the [`Packable`](UnpackError::Packable) variant.
    pub fn from_packable(err: impl Into<T>) -> Self {
        Self::Packable(err.into())
    }
}

impl<T, U> From<U> for UnpackError<T, U> {
    fn from(err: U) -> Self {
        Self::Unpacker(err)
    }
}

impl<U> UnpackError<Infallible, U> {
    /// Get the [`Packer`](UnpackError::Unpacker) variant if the [`Packable`](UnpackError::Packable) variant is
    /// [`Infallible`].
    pub fn into_unpacker_err(self) -> U {
        match self {
            Self::Packable(err) => match err {},
            Self::Unpacker(err) => err,
        }
    }
}

impl<T> UnpackError<T, Infallible> {
    /// Get the [`Packable`](UnpackError::Packable) variant if the [`Unpacker`](UnpackError::Unpacker) variant is
    /// [`Infallible`].
    pub fn into_packable_err(self) -> T {
        match self {
            Self::Packable(err) => err,
            Self::Unpacker(err) => match err {},
        }
    }
}

impl<T, U> fmt::Display for UnpackError<T, U>
where
    T: fmt::Display,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packable(err) => write!(f, "packable error while unpacking: {}", err),
            Self::Unpacker(err) => write!(f, "unpacker error while unpacking: {}", err),
        }
    }
}

#[cfg(feature = "std")]
impl<T, U> std::error::Error for UnpackError<T, U>
where
    T: std::error::Error,
    U: std::error::Error,
{
}

/// We cannot provide a [`From`] implementation because [`Infallible`] is not from this crate.
#[allow(clippy::from_over_into)]
impl Into<Infallible> for UnpackError<Infallible, Infallible> {
    fn into(self) -> Infallible {
        let (Self::Packable(err) | Self::Unpacker(err)) = self;
        match err {}
    }
}

/// Error type raised when an unknown tag is found while unpacking.
#[derive(Debug)]
pub struct UnknownTagError<T>(pub T);

#[cfg(feature = "std")]
impl<T> std::error::Error for UnknownTagError<T> where T: fmt::Display + fmt::Debug {}

impl<T> From<Infallible> for UnknownTagError<T> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

impl<T: fmt::Display> fmt::Display for UnknownTagError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown tag value {}", self.0)
    }
}

/// Error type to be raised when `&[u8]` does not have enough bytes to unpack something or when
/// [`SlicePacker`]('crate::packer::SlicePacker') does not have enough space to pack something.
#[derive(Debug)]
pub struct UnexpectedEOF {
    /// The required number of bytes.
    pub required: usize,
    /// The number of bytes the unpacker had or the number of bytes the packer can receive.
    pub had: usize,
}

#[cfg(feature = "std")]
impl std::error::Error for UnexpectedEOF {}

impl fmt::Display for UnexpectedEOF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not enough bytes, required {} but had {}", self.required, self.had)
    }
}
