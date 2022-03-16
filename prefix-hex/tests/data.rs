// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use prefix_hex::Error;

#[test]
fn array_decode() {
    assert_eq!(prefix_hex::decode("0xffffff"), Ok([255, 255, 255]));
}

#[test]
fn array_decode_invalid_hex() {
    assert_eq!(
        prefix_hex::decode::<[u8; 3]>("0x00000y"),
        Err(Error::InvalidHexCharacter { c: 'y', index: 5 })
    );
}

#[test]
fn array_decode_invalid_length_too_short() {
    assert_eq!(
        prefix_hex::decode::<[u8; 3]>("0x52fd6"),
        Err(Error::InvalidStringLengthSlice { expected: 6, actual: 5 })
    );
}

#[test]
fn array_decode_invalid_length_too_long() {
    assert_eq!(
        prefix_hex::decode::<[u8; 3]>("0x52fd643"),
        Err(Error::InvalidStringLengthSlice { expected: 6, actual: 7 })
    );
}

#[test]
fn array_decode_no_prefix() {
    assert_eq!(
        prefix_hex::decode::<[u8; 3]>("004200"),
        Err(Error::InvalidPrefix { c0: '0', c1: '0' })
    );
}

#[test]
fn array_decode_wrong_prefix() {
    assert_eq!(
        prefix_hex::decode::<[u8; 3]>("0yffffff"),
        Err(Error::InvalidPrefix { c0: '0', c1: 'y' })
    );
}

#[test]
fn vec_decode() {
    assert_eq!(prefix_hex::decode::<Vec<u8>>("0x000102").unwrap(), [0x0, 0x1, 0x2]);
}

#[test]
fn vec_decode_empty_string() {
    assert_eq!(prefix_hex::decode("0x"), Ok(vec![]));
}

#[test]
fn vec_decode_odd_length() {
    assert_eq!(prefix_hex::decode::<Vec<u8>>("0xf0f0f"), Err(Error::OddLength));
}