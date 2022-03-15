// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{format, string::String};

use crate::{strip_prefix, Error, FromHexPrefix, ToHexPrefix};

macro_rules! impl_from_hex {
    ($type:ty) => {
        impl FromHexPrefix for $type {
            fn from_hex_prefix(hex: &str) -> Result<Self, Error> {
                let hex = strip_prefix(hex)?;

                if hex.is_empty() {
                    return Err(Error::InvalidStringLength);
                }

                <$type>::from_str_radix(hex, 16).map_err(|_| {
                    // TODO: Proper error handling
                    Error::InvalidStringLength
                })
            }
        }

        impl ToHexPrefix for $type {
            fn to_hex_prefix(self) -> String {
                format!("0x{:x}", self)
            }
        }
    };
}

impl_from_hex!(primitive_types::U128);
impl_from_hex!(primitive_types::U256);
impl_from_hex!(primitive_types::U512);
