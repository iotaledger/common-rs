// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use core::{any::TypeId, convert::Infallible};

use packable::{error::UnknownTagError, Packable};

#[derive(Packable)]
pub struct Point {
    x: i32,
    y: u32,
}

#[derive(Packable)]
#[packable(tag_type = u8)]
pub enum Foo {
    #[packable(tag = 0)]
    Bar(u32),
    #[packable(tag = 1)]
    Baz { x: i32, y: i32 },
}

#[derive(Packable)]
pub struct Bar {
    foo: Foo,
}

fn main() {
    assert_eq!(
        TypeId::of::<Infallible>(),
        TypeId::of::<<Point as Packable>::UnpackError>()
    );

    assert_eq!(
        TypeId::of::<UnknownTagError<u8>>(),
        TypeId::of::<<Foo as Packable>::UnpackError>()
    );

    assert_eq!(
        TypeId::of::<UnknownTagError<u8>>(),
        TypeId::of::<<Bar as Packable>::UnpackError>()
    );
}
