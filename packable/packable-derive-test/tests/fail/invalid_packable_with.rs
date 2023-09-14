// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::Packable;

#[derive(Packable)]
#[packable(unpack_error = Impossible, with = Impossible::new)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub enum Impossible {}

impl Impossible {
    fn new() -> Self {
        unreachable!()
    }
}

impl From<Infallible> for Impossible {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

fn main() {}
