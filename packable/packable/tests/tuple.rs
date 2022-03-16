// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

macro_rules! repeat {
    () => {};
    ($first_val:expr, $($val:expr,)*) => {
        test_packable_tuple!($first_val, $($val,)*);
        repeat!($($val,)*);
    };
}

macro_rules! test_packable_tuple {
    ($($val:expr,)+) => {
        assert_eq!(
            common::generic_test(&($($val,)+)).0.len(),
            0 $( + core::mem::size_of_val(&$val))+,
        );
    };
}

#[test]
fn packable_tuple() {
    repeat!(
        8u8, 16u16, 32u32, 64u64, 32.0f32, 64.0f64, -8i8, -16i16, -32i32, -64i64, -32.0f32, -64.0f64,
    );
}
