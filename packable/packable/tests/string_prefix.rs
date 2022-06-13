// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

use packable::{
    bounded::{
        BoundedU16, BoundedU32, BoundedU64, BoundedU8, InvalidBoundedU16, InvalidBoundedU32, InvalidBoundedU64,
        InvalidBoundedU8, TryIntoBoundedU32Error,
    },
    error::UnpackError,
    prefix::{StringPrefix, UnpackPrefixError},
    PackableExt,
};

#[test]
fn packable_string_prefix_from_string_invalid_error() {
    // This measures 16 bytes.
    let string = String::from("yellow submarine");
    let prefixed = StringPrefix::<BoundedU32<1, 8>>::try_from(string);

    assert!(matches!(prefixed, Err(TryIntoBoundedU32Error::Invalid(16))));
}

#[test]
fn packable_string_prefix_from_string_truncated_error() {
    // This measures 257 bytes.
    let string = String::from(
        "In the town where I was born lived a man who sailed to sea and he told us of his life in the land of submarines. So we sailed on to the sun til we found a sea of green and we lived beneath the waves in our yellow submarine. We all live in a yellow submarine",
    );
    let prefixed = StringPrefix::<u8>::try_from(string);

    assert!(prefixed.is_err());
}

macro_rules! impl_packable_test_for_string_prefix {
    ($packable_string_prefix:ident, $packable_string_prefix_invalid_length:ident, $ty:ty) => {
        #[test]
        fn $packable_string_prefix() {
            assert_eq!(
                common::generic_test(&<StringPrefix<$ty>>::try_from(String::from("yellow submarine")).unwrap())
                    .0
                    .len(),
                core::mem::size_of::<$ty>() + 16 * core::mem::size_of::<u8>()
            );
        }
    };
}

macro_rules! impl_packable_test_for_bounded_string_prefix {
    ($packable_string_prefix:ident, $packable_string_prefix_invalid_length:ident, $ty:ty, $bounded:ident, $err:ident, $min:expr, $max:expr) => {
        #[test]
        fn $packable_string_prefix() {
            assert_eq!(
                common::generic_test(
                    &<StringPrefix<$bounded<$min, $max>>>::try_from(String::from("yellow submarine")).unwrap()
                )
                .0
                .len(),
                core::mem::size_of::<$ty>() + 16 * core::mem::size_of::<u8>()
            );
        }

        #[test]
        fn $packable_string_prefix_invalid_length() {
            const LEN: usize = $max + 1;

            let mut bytes = vec![0; LEN + 1];
            bytes[0] = LEN as u8;

            let prefixed = StringPrefix::<$bounded<$min, $max>>::unpack_verified(bytes);

            const LEN_AS_TY: $ty = LEN as $ty;

            assert!(matches!(
                prefixed,
                Err(UnpackError::Packable(UnpackPrefixError::Prefix($err(LEN_AS_TY)))),
            ));
        }
    };
}

impl_packable_test_for_string_prefix!(packable_string_prefix_u8, packable_string_prefix_invalid_length_u8, u8);
impl_packable_test_for_string_prefix!(
    packable_string_prefix_u16,
    packable_string_prefix_invalid_length_u16,
    u16
);
impl_packable_test_for_string_prefix!(
    packable_string_prefix_u32,
    packable_string_prefix_invalid_length_u32,
    u32
);
impl_packable_test_for_string_prefix!(
    packable_string_prefix_u64,
    packable_string_prefix_invalid_length_u64,
    u64
);

impl_packable_test_for_bounded_string_prefix!(
    packable_string_prefix_bounded_u8,
    packable_string_prefix_invalid_length_bounded_u8,
    u8,
    BoundedU8,
    InvalidBoundedU8,
    1,
    64
);
impl_packable_test_for_bounded_string_prefix!(
    packable_string_prefix_bounded_u16,
    packable_string_prefix_invalid_length_bounded_u16,
    u16,
    BoundedU16,
    InvalidBoundedU16,
    1,
    64
);
impl_packable_test_for_bounded_string_prefix!(
    packable_string_prefix_bounded_u32,
    packable_string_prefix_invalid_length_bounded_u32,
    u32,
    BoundedU32,
    InvalidBoundedU32,
    1,
    64
);
impl_packable_test_for_bounded_string_prefix!(
    packable_string_prefix_bounded_u64,
    packable_string_prefix_invalid_length_bounded_u64,
    u64,
    BoundedU64,
    InvalidBoundedU64,
    1,
    64
);
