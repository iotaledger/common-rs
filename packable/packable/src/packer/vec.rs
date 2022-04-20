// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::vec::Vec;
use core::convert::Infallible;

use crate::packer::Packer;

impl Packer for Vec<u8> {
    type Error = Infallible;

    #[inline]
    fn pack_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<(), Self::Error> {
        self.extend_from_slice(bytes.as_ref());
        Ok(())
    }

    #[inline]
    fn written_bytes(&self) -> Option<usize> {
        Some(self.len())
    }
}
