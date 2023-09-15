// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::Packable;

#[derive(Packable)]
#[packable(unpack_error = Infallible)]
pub struct Point {
    x: i32,
    y: i32,
}

fn main() {}
