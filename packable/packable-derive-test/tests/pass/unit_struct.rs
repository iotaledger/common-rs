// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::convert::Infallible;

use packable::Packable;

#[derive(Packable)]
#[packable(unpack_error = Infallible)]
pub struct Unit;

#[derive(Packable)]
#[packable(unpack_error = Infallible)]
pub struct RoundUnit();

fn main() {}
