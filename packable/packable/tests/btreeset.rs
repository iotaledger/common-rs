// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;

use packable::{
    error::UnpackError,
    set::{UnpackOrderedSetError, UnpackSetError},
    PackableExt,
};

mod common;

#[test]
fn packable_btreeset() {
    assert_eq!(
        common::generic_test(&BTreeSet::from([None, Some(0u32)])).0.len(),
        core::mem::size_of::<u64>()
            + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
            + core::mem::size_of::<u8>()
    );
}

#[test]
fn invalid_duplicate() {
    let bytes = [1, 2, 3, 3, 4];
    let bytes = Vec::from_iter(bytes.len().to_le_bytes().into_iter().chain(bytes));

    let prefixed = BTreeSet::<u8>::unpack_bytes_verified(bytes, &());

    println!("{prefixed:?}");

    assert!(matches!(
        prefixed,
        Err(UnpackError::Packable(UnpackOrderedSetError::Set(
            UnpackSetError::DuplicateItem(3u8)
        ))),
    ));
}

#[test]
fn invalid_unordered() {
    let bytes = [1, 2, 4, 3];
    let bytes = Vec::from_iter(bytes.len().to_le_bytes().into_iter().chain(bytes));

    let prefixed = BTreeSet::<u8>::unpack_bytes_verified(bytes, &());

    assert!(matches!(
        prefixed,
        Err(UnpackError::Packable(UnpackOrderedSetError::Unordered)),
    ));
}
