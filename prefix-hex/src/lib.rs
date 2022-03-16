// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `hex-prefix` crates offers encoding and decoding of hex strings with a `0x` prefix.
//!
//! Its API aims to mimic that of the [`hex`](https://docs.rs/hex/latest/hex/) crate, which we also use internally.
//!
//! This crate is compatible with the hex encoding rules of the [Ethereum RPC API](https://eth.wiki/json-rpc/API#hex-value-encoding).

#![no_std]

extern crate alloc;

mod error;

mod data;

#[cfg(feature = "primitive-types")]
mod primitive_types;

use alloc::string::String;

pub use error::Error;

/// Tries to decode an hexadecimal encoded string with a `0x` prefix.
pub trait FromHexPrefix: Sized {
    /// Tries to decode an hexadecimal encoded string with a `0x` prefix.
    fn from_hex_prefix(hex: &str) -> Result<Self, Error>;
}

// TODO: Maybe introduce `handle_error` with `#[cold]` attribute.
fn strip_prefix(hex: &str) -> Result<&str, Error> {
    if hex.starts_with("0x") {
        Ok(&hex[2..])
    } else if hex.len() < 2 {
        Err(Error::InvalidStringLength)
    } else {
        let mut chars = hex.chars();
        // Safety the following two operations are safe because we checked for the `hex.len()` in a previous branch.
        let c0 = chars.next().unwrap();
        let c1 = chars.next().unwrap();
        Err(Error::InvalidPrefix { c0, c1 })
    }
}

/// Encodes data into an hexadecimal encoded string with a `0x` prefix.
pub trait ToHexPrefix {
    /// Encodes data into an hexadecimal encoded string with a `0x` prefix.
    fn to_hex_prefix(self) -> String;
}

///
pub fn decode<T: FromHexPrefix>(hex: &str) -> Result<T, Error> {
    T::from_hex_prefix(hex)
}

///
pub fn encode<T: ToHexPrefix>(value: T) -> String {
    ToHexPrefix::to_hex_prefix(value)
}

#[cfg(test)]
mod test {
    use alloc::string::ToString;

    #[test]
    fn endianess() {
        // TODO check endianess again against eth hex encoding
        let x = 10000u64.to_le_bytes();
        let encoded = super::encode(x);
        assert_eq!(encoded, "0x1027000000000000".to_string());
        let decoded: [u8; 8] = super::decode(&encoded).unwrap();
        assert_eq!(decoded, [0x10, 0x27, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
        assert_eq!(x, decoded);
    }
}
