// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{format, string::String, vec::Vec};

use crate::{strip_prefix, Error, FromHexPrefixed, ToHexPrefixed};

impl FromHexPrefixed for Vec<u8> {
    fn from_hex_prefixed(hex: &str) -> Result<Self, Error> {
        let hex = strip_prefix(hex)?;
        hex::decode(hex).map_err(Into::into)
    }
}

impl ToHexPrefixed for Vec<u8> {
    fn to_hex_prefixed(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

impl<const N: usize> FromHexPrefixed for [u8; N]
where
    Self: hex::FromHex,
{
    fn from_hex_prefixed(hex: &str) -> Result<Self, Error> {
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

impl<const N: usize> ToHexPrefixed for [u8; N]
where
    Self: hex::ToHex,
{
    fn to_hex_prefixed(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

impl<const N: usize> ToHexPrefixed for &[u8; N]
where
    [u8; N]: hex::ToHex,
{
    fn to_hex_prefixed(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

impl ToHexPrefixed for &[u8] {
    fn to_hex_prefixed(self) -> String {
        format!("0x{}", hex::encode(self))
    }
}
