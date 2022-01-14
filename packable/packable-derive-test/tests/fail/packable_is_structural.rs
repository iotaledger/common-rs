// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use packable::Packable;

use core::convert::Infallible;

struct NonPackable;

#[derive(Packable)]
#[packable(unpack_error = Infallible)]
pub struct Wrapper(NonPackable);

fn main() {}
