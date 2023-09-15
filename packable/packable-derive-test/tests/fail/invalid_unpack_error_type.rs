// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::Packable;

#[derive(Packable)]
#[packable(unpack_error = 1)]
pub struct Point {
    x: u32,
    y: u32,
}

fn main() {}
