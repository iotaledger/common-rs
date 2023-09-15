// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate std;

use std::{
    io::{self, Read},
    ops::Deref,
};

use crate::unpacker::Unpacker;

/// An [`Unpacker`] backed by [`Read`].
pub struct IoUnpacker<R: Read>(R);

impl<R: Read> IoUnpacker<R> {
    /// Creates a new [`Unpacker`] from a value that implements [`Read`].
    pub fn new(reader: R) -> Self {
        Self(reader)
    }

    /// Consumes the value to return the inner value that implements [`Read`].
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R: Read> Deref for IoUnpacker<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Read> Unpacker for IoUnpacker<R> {
    type Error = io::Error;

    #[inline]
    fn unpack_bytes<B: AsMut<[u8]>>(&mut self, mut bytes: B) -> Result<(), Self::Error> {
        self.0.read_exact(bytes.as_mut())
    }
}
