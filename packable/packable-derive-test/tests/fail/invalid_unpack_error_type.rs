// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use packable::Packable;

use core::convert::Infallible;

#[derive(Packable)]
#[packable(unpack_error = 1)]
pub struct Point {
    x: u32,
    y: u32,
}

fn main() {}
