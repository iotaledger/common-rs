// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::UnexpectedEOF, unpacker::Unpacker};

/// A [`Unpacker`] backed by a `&mut [u8]`.
#[repr(transparent)]
pub struct SliceUnpacker<'a> {
    slice: &'a [u8],
}

impl<'a> SliceUnpacker<'a> {
    /// Creates a new [`SliceUnpacker`] from a `&[u8]`.
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice }
    }
}

impl<'u> Unpacker for SliceUnpacker<'u> {
    type Error = UnexpectedEOF;

    #[inline]
    fn unpack_bytes<B: AsMut<[u8]>>(&mut self, mut bytes: B) -> Result<(), Self::Error> {
        let slice = bytes.as_mut();
        let len = slice.len();

        if self.slice.len() >= len {
            let (head, tail) = self.slice.split_at(len);
            self.slice = tail;
            slice.copy_from_slice(head);
            Ok(())
        } else {
            Err(UnexpectedEOF {
                required: len,
                had: self.slice.len(),
            })
        }
    }

    #[inline]
    fn ensure_bytes(&self, len: usize) -> Result<(), Self::Error> {
        if self.slice.len() < len {
            Err(UnexpectedEOF {
                required: len,
                had: self.slice.len(),
            })
        } else {
            Ok(())
        }
    }
}
