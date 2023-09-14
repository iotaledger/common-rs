// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::packer::Packer;

/// A [`Packer`] able to keep count of the number of written bytes.
pub struct CounterPacker<P: Packer> {
    inner: P,
    counter: usize,
}

impl<P: Packer> CounterPacker<P> {
    /// Creates a new [`CounterPacker`].
    #[inline]
    pub fn new(packer: P) -> Self {
        Self {
            inner: packer,
            counter: 0,
        }
    }

    /// Returns the number of written bytes.
    #[inline]
    pub fn counter(&self) -> usize {
        self.counter
    }

    /// Consumes the value to return the inner [`Packer`].
    #[inline]
    pub fn into_inner(self) -> P {
        self.inner
    }
}

impl<P: Packer> Packer for CounterPacker<P> {
    type Error = P::Error;

    fn pack_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<(), Self::Error> {
        let bytes = bytes.as_ref();
        let len = bytes.len();

        self.inner.pack_bytes(bytes)?;
        self.counter += len;

        Ok(())
    }

    #[inline]
    fn written_bytes(&self) -> Option<usize> {
        Some(self.counter)
    }
}
