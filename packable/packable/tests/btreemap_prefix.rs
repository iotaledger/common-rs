// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

use std::collections::BTreeMap;

use packable::{
    bounded::{
        BoundedU16, BoundedU32, BoundedU64, BoundedU8, InvalidBoundedU16, InvalidBoundedU32, InvalidBoundedU64,
        InvalidBoundedU8, TryIntoBoundedU32Error,
    },
    error::UnpackError,
    map::{UnpackMapError, UnpackOrderedMapError},
    prefix::BTreeMapPrefix,
    PackableExt,
};

#[test]
fn btreemap_prefix_from_btreemap_invalid_error() {
    let set = BTreeMap::from_iter((0..16).zip(4..20));
    let prefixed = BTreeMapPrefix::<u8, u8, BoundedU32<1, 8>>::try_from(set);

    assert!(matches!(prefixed, Err(TryIntoBoundedU32Error::Invalid(16))));
}

#[test]
fn btreemap_prefix_from_btreemap_truncated_error() {
    let set = BTreeMap::from_iter((0..257).zip(0..257));
    let prefixed = BTreeMapPrefix::<u16, u16, u8>::try_from(set);

    assert!(prefixed.is_err());
}

macro_rules! impl_packable_test_for_btreemap_prefix {
    (
        $packable_btreemap_prefix:ident,
        $packable_btreemap_prefix_duplicate:ident,
        $packable_btreemap_prefix_unordered:ident,
        $ty:ty) => {
        #[test]
        fn $packable_btreemap_prefix() {
            assert_eq!(
                common::generic_test(
                    &<BTreeMapPrefix<Option<u32>, u32, $ty>>::try_from(BTreeMap::from([(Some(0u32), 0), (Some(1), 5), (None, 10)])).unwrap()
                )
                .0
                .len(),
                core::mem::size_of::<$ty>()
                    + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
                    + 2 * (core::mem::size_of::<u8>() + 2 * core::mem::size_of::<u32>())
            );
        }

        #[test]
        fn $packable_btreemap_prefix_duplicate() {
            const LEN: usize = 64;
            const LEN_AS_TY: $ty = LEN as $ty;

            let mut bytes = (0u8..LEN as u8).zip((0u8..LEN as u8).rev()).flat_map(|(k, v)| [k, v]).collect::<Vec<_>>();
            bytes[LEN - 2] = bytes[LEN - 4];
            let dup = bytes[LEN - 2];

            let bytes = Vec::from_iter(LEN_AS_TY.to_le_bytes().into_iter().chain(bytes));

            let prefixed = BTreeMapPrefix::<u8, u8, $ty>::unpack_verified(bytes, &());

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackOrderedMapError::Map(
                    UnpackMapError::DuplicateKey(d)
                ))) if d == dup
            ));
        }

        #[test]
        fn $packable_btreemap_prefix_unordered() {
            const LEN: usize = 64;
            const LEN_AS_TY: $ty = LEN as $ty;

            let mut bytes = (0u8..LEN as u8).zip((0u8..LEN as u8).rev()).flat_map(|(k, v)| [k, v]).collect::<Vec<_>>();
            // Swap the key and value at index 0 and 1
            let (s1, s2) = bytes.split_at_mut(2);
            s1.swap_with_slice(&mut s2[..2]);

            let bytes = Vec::from_iter(LEN_AS_TY.to_le_bytes().into_iter().chain(bytes));

            let prefixed = BTreeMapPrefix::<u8, u8, $ty>::unpack_verified(bytes, &());

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackOrderedMapError::Unordered)),
            ));
        }
    };
}

macro_rules! impl_packable_test_for_bounded_btreemap_prefix {
    (
        $packable_btreemap_prefix:ident,
        $packable_btreemap_prefix_invalid_length:ident,
        $packable_btreemap_prefix_duplicate:ident,
        $packable_btreemap_prefix_unordered:ident,
        $ty:ty,
        $bounded:ident,
        $err:ident,
        $min:expr,
        $max:expr) => {
        #[test]
        fn $packable_btreemap_prefix() {
            assert_eq!(
                common::generic_test(
                    &<BTreeMapPrefix<Option<u32>, u32, $bounded<$min, $max>>>::try_from(BTreeMap::from([(Some(0u32), 0), (Some(1), 5), (None, 10)]))
                        .unwrap()
                )
                .0
                .len(),
                core::mem::size_of::<$ty>()
                    + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
                    + 2 * (core::mem::size_of::<u8>() + 2 * core::mem::size_of::<u32>())
            );
        }

        #[test]
        fn $packable_btreemap_prefix_invalid_length() {
            const LEN: usize = $max + 1;
            const LEN_AS_TY: $ty = LEN as $ty;

            let bytes = Vec::from_iter(LEN_AS_TY.to_le_bytes().into_iter().chain(core::iter::repeat(0).take(2 * (LEN + 1))));

            let prefixed = BTreeMapPrefix::<u8, u8, $bounded<$min, $max>>::unpack_verified(bytes, &());

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackOrderedMapError::Map(
                    UnpackMapError::Prefix($err(LEN_AS_TY))
                ))),
            ));
        }

        #[test]
        fn $packable_btreemap_prefix_duplicate() {
            const LEN: usize = $max;
            const LEN_AS_TY: $ty = LEN as $ty;

            let mut bytes = (0u8..LEN as u8).zip((0u8..LEN as u8).rev()).flat_map(|(k, v)| [k, v]).collect::<Vec<_>>();
            bytes[LEN - 2] = bytes[LEN - 4];
            let dup = bytes[LEN - 2];

            let bytes = Vec::from_iter(LEN_AS_TY.to_le_bytes().into_iter().chain(bytes));

            let prefixed = BTreeMapPrefix::<u8, u8, $bounded<$min, $max>>::unpack_verified(bytes, &());

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackOrderedMapError::Map(
                    UnpackMapError::DuplicateKey(d)
                ))) if d == dup
            ));
        }

        #[test]
        fn $packable_btreemap_prefix_unordered() {
            const LEN: usize = $max;
            const LEN_AS_TY: $ty = LEN as $ty;

            let mut bytes = (0u8..LEN as u8).zip((0u8..LEN as u8).rev()).flat_map(|(k, v)| [k, v]).collect::<Vec<_>>();
            // Swap the key and value at index 0 and 1
            let (s1, s2) = bytes.split_at_mut(2);
            s1.swap_with_slice(&mut s2[..2]);

            let bytes = Vec::from_iter(LEN_AS_TY.to_le_bytes().into_iter().chain(bytes));

            let prefixed = BTreeMapPrefix::<u8, u8, $bounded<$min, $max>>::unpack_verified(bytes, &());

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackOrderedMapError::Unordered)),
            ));
        }
    };
}

impl_packable_test_for_btreemap_prefix!(
    packable_btreemap_prefix_u8,
    packable_btreemap_prefix_duplicate_u8,
    packable_btreemap_prefix_unordered_u8,
    u8
);
impl_packable_test_for_btreemap_prefix!(
    packable_btreemap_prefix_u16,
    packable_btreemap_prefix_duplicate_u16,
    packable_btreemap_prefix_unordered_u16,
    u16
);
impl_packable_test_for_btreemap_prefix!(
    packable_btreemap_prefix_u32,
    packable_btreemap_prefix_duplicate_u32,
    packable_btreemap_prefix_unordered_u32,
    u32
);
impl_packable_test_for_btreemap_prefix!(
    packable_btreemap_prefix_u64,
    packable_btreemap_prefix_duplicate_u64,
    packable_btreemap_prefix_unordered_u64,
    u64
);

impl_packable_test_for_bounded_btreemap_prefix!(
    packable_btreemap_prefix_bounded_u8,
    packable_btreemap_prefix_invalid_length_bounded_u8,
    packable_btreemap_prefix_duplicate_bounded_u8,
    packable_btreemap_prefix_unordered_bounded_u8,
    u8,
    BoundedU8,
    InvalidBoundedU8,
    1,
    64
);
impl_packable_test_for_bounded_btreemap_prefix!(
    packable_btreemap_prefix_bounded_u16,
    packable_btreemap_prefix_invalid_length_bounded_u16,
    packable_btreemap_prefix_duplicate_bounded_u16,
    packable_btreemap_prefix_unordered_bounded_u16,
    u16,
    BoundedU16,
    InvalidBoundedU16,
    1,
    64
);
impl_packable_test_for_bounded_btreemap_prefix!(
    packable_btreemap_prefix_bounded_u32,
    packable_btreemap_prefix_invalid_length_bounded_u32,
    packable_btreemap_prefix_duplicate_bounded_u32,
    packable_btreemap_prefix_unordered_bounded_u32,
    u32,
    BoundedU32,
    InvalidBoundedU32,
    1,
    64
);
impl_packable_test_for_bounded_btreemap_prefix!(
    packable_btreemap_prefix_bounded_u64,
    packable_btreemap_prefix_invalid_length_bounded_u64,
    packable_btreemap_prefix_duplicate_bounded_u64,
    packable_btreemap_prefix_unordered_bounded_u64,
    u64,
    BoundedU64,
    InvalidBoundedU64,
    1,
    64
);
