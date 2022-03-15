// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate std;

use std::{
    io::{self, Write},
    ops::Deref,
};

use crate::packer::Packer;

/// A [`Packer`] backed by [`Write`].
pub struct IoPacker<W: Write>(W);

impl<W: Write> IoPacker<W> {
    /// Creates a new [`Packer`] from a value that implements [`Write`].
    pub fn new(writer: W) -> Self {
        Self(writer)
    }

    /// Consumes the value to return the inner value that implements [`Write`].
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: Write> Deref for IoPacker<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<W: Write> Packer for IoPacker<W> {
    type Error = io::Error;

    #[inline]
    fn pack_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<(), Self::Error> {
        self.0.write_all(bytes.as_ref())
    }
}
