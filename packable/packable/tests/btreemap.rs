// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use packable::{
    error::UnpackError,
    map::{UnpackMapError, UnpackOrderedMapError},
    PackableExt,
};

mod common;

#[test]
fn packable_btreemap() {
    assert_eq!(
        common::generic_test(&BTreeMap::from([(None, 1u32), (Some(0u32), 1), (Some(1u32), 2)]))
            .0
            .len(),
        core::mem::size_of::<u64>()
            + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
            + 2 * (core::mem::size_of::<u8>() + 2 * core::mem::size_of::<u32>())
    );
}

#[test]
fn invalid_duplicate() {
    let bytes = [(1, 5), (2, 4), (3, 3), (3, 2), (4, 1)];
    let bytes = Vec::from_iter(
        bytes
            .len()
            .to_le_bytes()
            .into_iter()
            .chain(bytes.into_iter().flat_map(|(k, v)| [k, v])),
    );

    let prefixed = BTreeMap::<u8, u8>::unpack_verified(bytes, &());

    assert!(matches!(
        prefixed,
        Err(UnpackError::Packable(UnpackOrderedMapError::Map(
            UnpackMapError::DuplicateKey(3u8)
        ))),
    ));
}

#[test]
fn invalid_unordered() {
    let bytes = [(1, 4), (2, 3), (4, 1), (3, 2)];
    let bytes = Vec::from_iter(
        bytes
            .len()
            .to_le_bytes()
            .into_iter()
            .chain(bytes.into_iter().flat_map(|(k, v)| [k, v])),
    );

    let prefixed = BTreeMap::<u8, u8>::unpack_verified(bytes, &());

    assert!(matches!(
        prefixed,
        Err(UnpackError::Packable(UnpackOrderedMapError::Unordered)),
    ));
}
