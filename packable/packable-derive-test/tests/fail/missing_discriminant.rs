// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use packable::Packable;

#[derive(Packable)]
#[repr(u8)]
#[packable(tag_type = u8)]
pub enum A {
    B = 0,
    C,
}

fn main() {}
