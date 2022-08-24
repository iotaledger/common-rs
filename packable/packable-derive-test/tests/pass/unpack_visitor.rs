// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::{borrow::Borrow, convert::Infallible};

use packable::{error::UnknownTagError, Packable};

#[derive(Default)]
pub struct Visitor {
    inner: (),
}

impl Borrow<()> for Visitor {
    fn borrow(&self) -> &() {
        &self.inner
    }
}

#[derive(Packable)]
#[packable(unpack_error = Infallible)]
#[packable(unpack_visitor = Visitor)]
pub struct Point(i32, i32);

#[derive(Packable)]
#[packable(tag_type = u8)]
#[packable(unpack_error = UnknownTagError<u8>)]
#[packable(unpack_visitor = Visitor)]
pub enum Answer {
    #[packable(tag = 0)]
    Yes,
    #[packable(tag = 1)]
    No,
}

fn main() {}
