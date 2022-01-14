// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use packable::Packable;

use core::convert::Infallible;

#[derive(Packable)]
#[packable(unknown_ident = true)]
pub struct Unit;

fn main() {}
