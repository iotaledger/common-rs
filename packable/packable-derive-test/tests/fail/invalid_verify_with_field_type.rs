// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::{
    error::{UnknownTagError, UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

#[derive(Debug)]
pub struct PickyError(u8);

impl From<Infallible> for PickyError {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

fn verify_value(&value: &u64) -> Result<(), PickyError> {
    if value == 42 {
        Ok(())
    } else {
        Err(PickyError(value as u8))
    }
}

#[derive(Packable)]
#[packable(unpack_error = PickyError)]
pub struct Picky(#[packable(verify_with = verify_value)] u8);

fn main() {}
