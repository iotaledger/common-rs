// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

use std::collections::BTreeSet;

use packable::{
    bounded::{
        BoundedU16, BoundedU32, BoundedU64, BoundedU8, InvalidBoundedU16, InvalidBoundedU32, InvalidBoundedU64,
        InvalidBoundedU8, TryIntoBoundedU32Error,
    },
    error::UnpackError,
    prefix::BTreeSetPrefix,
    set::UnpackSetError,
    PackableExt,
};

#[test]
fn btreeset_prefix_from_btreeset_invalid_error() {
    let set = BTreeSet::from_iter(0..16);
    let prefixed = BTreeSetPrefix::<u8, BoundedU32<1, 8>>::try_from(set);

    assert!(matches!(prefixed, Err(TryIntoBoundedU32Error::Invalid(16))));
}

#[test]
fn btreeset_prefix_from_btreeset_truncated_error() {
    let set = BTreeSet::from_iter(0..257);
    let prefixed = BTreeSetPrefix::<u16, u8>::try_from(set);

    assert!(prefixed.is_err());
}

macro_rules! impl_packable_test_for_btreeset_prefix {
    ($packable_btreeset_prefix:ident, $packable_btreeset_prefix_invalid_length:ident, $ty:ty) => {
        #[test]
        fn $packable_btreeset_prefix() {
            assert_eq!(
                common::generic_test(
                    &<BTreeSetPrefix<Option<u32>, $ty>>::try_from(BTreeSet::from([Some(0u32), None])).unwrap()
                )
                .0
                .len(),
                core::mem::size_of::<$ty>()
                    + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
                    + core::mem::size_of::<u8>()
            );
        }
    };
}

macro_rules! impl_packable_test_for_bounded_btreeset_prefix {
    ($packable_btreeset_prefix:ident, $packable_btreeset_prefix_invalid_length:ident, $ty:ty, $bounded:ident, $err:ident, $min:expr, $max:expr) => {
        #[test]
        fn $packable_btreeset_prefix() {
            assert_eq!(
                common::generic_test(
                    &<BTreeSetPrefix<Option<u32>, $bounded<$min, $max>>>::try_from(BTreeSet::from([Some(0u32), None]))
                        .unwrap()
                )
                .0
                .len(),
                core::mem::size_of::<$ty>()
                    + (core::mem::size_of::<u8>() + core::mem::size_of::<u32>())
                    + core::mem::size_of::<u8>()
            );
        }

        #[test]
        fn $packable_btreeset_prefix_invalid_length() {
            const LEN: usize = $max + 1;

            let mut bytes = [0u8; LEN + 1];
            bytes[0] = LEN as u8;

            let prefixed = BTreeSetPrefix::<u8, $bounded<$min, $max>>::unpack_verified(bytes, &());

            const LEN_AS_TY: $ty = LEN as $ty;

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackSetError::Prefix($err(LEN_AS_TY)))),
            ));
        }
    };
}

impl_packable_test_for_btreeset_prefix!(
    packable_btreeset_prefix_u8,
    packable_btreeset_prefix_invalid_length_u8,
    u8
);
impl_packable_test_for_btreeset_prefix!(
    packable_btreeset_prefix_u16,
    packable_btreeset_prefix_invalid_length_u16,
    u16
);
impl_packable_test_for_btreeset_prefix!(
    packable_btreeset_prefix_u32,
    packable_btreeset_prefix_invalid_length_u32,
    u32
);
impl_packable_test_for_btreeset_prefix!(
    packable_btreeset_prefix_u64,
    packable_btreeset_prefix_invalid_length_u64,
    u64
);

impl_packable_test_for_bounded_btreeset_prefix!(
    packable_btreeset_prefix_bounded_u8,
    packable_btreeset_prefix_invalid_length_bounded_u8,
    u8,
    BoundedU8,
    InvalidBoundedU8,
    1,
    64
);
impl_packable_test_for_bounded_btreeset_prefix!(
    packable_btreeset_prefix_bounded_u16,
    packable_btreeset_prefix_invalid_length_bounded_u16,
    u16,
    BoundedU16,
    InvalidBoundedU16,
    1,
    64
);
impl_packable_test_for_bounded_btreeset_prefix!(
    packable_btreeset_prefix_bounded_u32,
    packable_btreeset_prefix_invalid_length_bounded_u32,
    u32,
    BoundedU32,
    InvalidBoundedU32,
    1,
    64
);
impl_packable_test_for_bounded_btreeset_prefix!(
    packable_btreeset_prefix_bounded_u64,
    packable_btreeset_prefix_invalid_length_bounded_u64,
    u64,
    BoundedU64,
    InvalidBoundedU64,
    1,
    64
);
