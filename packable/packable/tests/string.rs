// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

#[test]
fn packable_string() {
    assert_eq!(
        common::generic_test(&"yellow submarine".to_owned()).0.len(),
        core::mem::size_of::<u64>() + 16 * core::mem::size_of::<u8>()
    );
}
