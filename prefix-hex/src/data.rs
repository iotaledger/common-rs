// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{format, string::String, vec::Vec};

use crate::{strip_prefix, Error, FromHexPrefix, ToHexPrefix};

impl FromHexPrefix for Vec<u8> {
    fn from_hex_prefix(hex: &str) -> Result<Self, Error> {
        let hex = strip_prefix(hex)?;
        hex::decode(hex).map_err(|e| -> Error { e.into() })
    }
}

impl ToHexPrefix for Vec<u8> {
    fn to_hex_prefix(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

impl<const N: usize> FromHexPrefix for [u8; N]
where
    Self: hex::FromHex,
{
    fn from_hex_prefix(hex: &str) -> Result<Self, Error> {
        let hex = strip_prefix(hex)?;
        let mut buffer = [0; N];
        hex::decode_to_slice(hex, &mut buffer).map_err(|e| match e {
            hex::FromHexError::InvalidStringLength | hex::FromHexError::OddLength => Error::InvalidStringLengthSlice {
                expected: N * 2,
                actual: hex.len(),
            },
            _ => e.into(),
        })?;
        Ok(buffer)
    }
}

impl<const N: usize> ToHexPrefix for [u8; N]
where
    Self: hex::ToHex,
{
    fn to_hex_prefix(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

impl ToHexPrefix for &[u8] {
    fn to_hex_prefix(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}
