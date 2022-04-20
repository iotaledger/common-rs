// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A module to unpack any value that implements [`Packable`](crate::Packable).
//!
//! The [`Unpacker`] trait represents types that can be used to read bytes from it. It can be thought as a `no_std`
//! friendly alternative to the [`Read`](std::io::Read) trait.

mod counter;
#[cfg(feature = "io")]
mod io;
mod slice;

pub use counter::CounterUnpacker;
#[cfg(feature = "io")]
pub use io::IoUnpacker;

/// A type that can unpack any value that implements [`Packable`](crate::Packable).
pub trait Unpacker: Sized {
    /// An error type representing any error related to reading bytes.
    type Error;

    /// Reads a sequence of bytes from the [`Unpacker`]. This sequence must be long enough to fill `bytes` completely.
    /// This method **must** fail if the unpacker does not have enough bytes to fulfill the request.
    fn unpack_bytes<B: AsMut<[u8]>>(&mut self, bytes: B) -> Result<(), Self::Error>;

    /// Tries to guarantee that the [`Unpacker`] has at least `len` bytes.
    ///
    /// This method **must** fail if and only if it is certain that there are not enough bytes and
    /// it is allowed to return `Ok(())` in any other case.
    #[inline]
    fn ensure_bytes(&self, _len: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Returns the exact number of read bytes if possible.
    #[inline]
    fn read_bytes(&self) -> Option<usize> {
        None
    }
}

impl<U: Unpacker + ?Sized> Unpacker for &mut U {
    type Error = U::Error;

    #[inline]
    fn unpack_bytes<B: AsMut<[u8]>>(&mut self, bytes: B) -> Result<(), Self::Error> {
        U::unpack_bytes(*self, bytes)
    }

    #[inline]
    fn ensure_bytes(&self, len: usize) -> Result<(), Self::Error> {
        U::ensure_bytes(*self, len)
    }

    #[inline]
    fn read_bytes(&self) -> Option<usize> {
        U::read_bytes(*self)
    }
}
