// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and utilities used to pack and unpack dynamic sequences of values with restricted length prefixes.

mod boxed;
mod string;
mod vec;

use core::{convert::Infallible, fmt};

pub use boxed::BoxedSlicePrefix;
pub use string::StringPrefix;
pub use vec::VecPrefix;

/// Semantic error raised while unpacking dynamically-sized sequences.
#[derive(Debug)]
pub enum UnpackPrefixError<T, E> {
    /// Semantic error raised while unpacking an item of the sequence. Typically this is
    /// [`Packable::UnpackError`](crate::Packable::UnpackError).
    Item(T),
    /// Semantic error raised when the length prefix cannot be unpacked.
    Prefix(E),
}

impl<T: fmt::Display, E: fmt::Display> fmt::Display for UnpackPrefixError<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Item(err) => write!(f, "cannot unpack item: {}", err),
            Self::Prefix(err) => write!(f, "cannot unpack prefix: {}", err),
        }
    }
}

#[cfg(feature = "std")]
impl<T, E> std::error::Error for UnpackPrefixError<T, E>
where
    T: std::error::Error,
    E: std::error::Error,
{
}

impl<E> UnpackPrefixError<Infallible, E> {
    /// Projects the value to the [`Prefix`](UnpackPrefixError::Prefix) variant.
    pub fn into_prefix_err(self) -> E {
        match self {
            Self::Item(err) => match err {},
            Self::Prefix(err) => err,
        }
    }
}

impl<T> UnpackPrefixError<T, Infallible> {
    /// Projects the value to the [`Item`](UnpackPrefixError::Item) variant.
    pub fn into_item_err(self) -> T {
        match self {
            Self::Item(err) => err,
            Self::Prefix(err) => match err {},
        }
    }
}

/// We cannot provide a [`From`] implementation because [`Infallible`] is not from this crate.
#[allow(clippy::from_over_into)]
impl Into<Infallible> for UnpackPrefixError<Infallible, Infallible> {
    fn into(self) -> Infallible {
        let (Self::Item(err) | Self::Prefix(err)) = self;
        match err {}
    }
}

impl<T, E> UnpackPrefixError<T, E> {
    /// Returns the contained [`Item`](UnpackPrefixError::Item) value or computes it from a closure.
    pub fn unwrap_item_err_or_else<V: Into<T>>(self, f: impl FnOnce(E) -> V) -> T {
        match self {
            Self::Item(err) => err,
            Self::Prefix(err) => f(err).into(),
        }
    }
}

impl<T, E> From<Infallible> for UnpackPrefixError<T, E> {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}
