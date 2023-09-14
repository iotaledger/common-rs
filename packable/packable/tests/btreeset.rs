// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;

mod common;

#[test]
fn packable_btreeset() {
    assert_eq!(
        common::generic_test(&BTreeSet::from([Some(0u32), None])).0.len(),
        core::mem::size_of::<u64>()
            + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
            + core::mem::size_of::<u8>()
    );
}
