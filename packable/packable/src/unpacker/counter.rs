// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::unpacker::Unpacker;

/// An [`Unpacker`] able to keep count of the number of read bytes.
pub struct CounterUnpacker<U: Unpacker> {
    inner: U,
    counter: usize,
}

impl<U: Unpacker> CounterUnpacker<U> {
    /// Creates a new [`CounterUnpacker`].
    #[inline]
    pub fn new(unpacker: U) -> Self {
        Self {
            inner: unpacker,
            counter: 0,
        }
    }

    /// Returns the number of read bytes.
    #[inline]
    pub fn counter(&self) -> usize {
        self.counter
    }

    /// Consumes the value to return the inner [`Unpacker`].
    #[inline]
    pub fn into_inner(self) -> U {
        self.inner
    }
}

impl<U: Unpacker> Unpacker for CounterUnpacker<U> {
    type Error = U::Error;

    fn unpack_bytes<B: AsMut<[u8]>>(&mut self, mut bytes: B) -> Result<(), Self::Error> {
        let bytes = bytes.as_mut();
        let len = bytes.len();

        self.inner.unpack_bytes(bytes)?;
        self.counter += len;

        Ok(())
    }

    #[inline]
    fn ensure_bytes(&self, len: usize) -> Result<(), Self::Error> {
        self.inner.ensure_bytes(len)
    }

    #[inline]
    fn read_bytes(&self) -> Option<usize> {
        Some(self.counter)
    }
}
