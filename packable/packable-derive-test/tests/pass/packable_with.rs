// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use packable::Packable;

use core::convert::Infallible;

#[derive(Packable)]
#[packable(unpack_error = Impossible, with = Impossible::from)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub enum Impossible {}

impl From<Infallible> for Impossible {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

fn main() {}
