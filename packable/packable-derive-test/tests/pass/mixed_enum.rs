// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::{error::UnknownTagError, Packable};

#[derive(Packable)]
#[packable(tag_type = u8)]
#[packable(unpack_error = UnknownTagError<u8>)]
pub enum Foo {
    #[packable(tag = 0)]
    Bar(u32),
    #[packable(tag = 1)]
    Baz { x: i32, y: i32 },
}

fn main() {}
