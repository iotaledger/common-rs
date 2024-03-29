// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports, unreachable_patterns)]

use packable::Packable;

#[derive(Packable)]
#[repr(u8)]
#[packable(tag_type = u8)]
pub enum A {
    B = 0,
    #[packable(tag = 0)]
    C,
}

fn main() {}
