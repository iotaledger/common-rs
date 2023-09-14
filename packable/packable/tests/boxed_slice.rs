// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

#[test]
fn packable_boxed_slice() {
    assert_eq!(
        common::generic_test(&vec![Some(0u32), None].into_boxed_slice()).0.len(),
        core::mem::size_of::<u64>()
            + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
            + core::mem::size_of::<u8>()
    );
}
