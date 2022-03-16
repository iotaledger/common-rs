// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{format, string::String};

use crate::{strip_prefix, Error, FromHexPrefix, ToHexPrefix};

#[rustfmt::skip]
fn is_hex_character(c: char) -> bool {
    matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
                'A' | 'B' | 'C' | 'D' | 'E' | 'F')
}

macro_rules! impl_from_to_hex {
    ($type:ty) => {
        impl FromHexPrefix for $type {
            fn from_hex_prefix(hex: &str) -> Result<Self, Error> {
                let hex = strip_prefix(hex)?;

                if hex.is_empty() {
                    return Err(Error::InvalidStringLength);
                }

                <$type>::from_str_radix(hex, 16).map_err(|error| match error.kind() {
                    uint::FromStrRadixErrKind::InvalidCharacter => {
                        if let Some((index, c)) = hex.chars().enumerate().find(|(_, c)| !is_hex_character(*c)) {
                            Error::InvalidHexCharacter { c, index }
                        } else {
                            unreachable!()
                        }
                    }
                    uint::FromStrRadixErrKind::InvalidLength => Error::InvalidStringLength,
                    _ => unreachable!(),
                })
            }
        }

        impl ToHexPrefix for $type {
            fn to_hex_prefix(self) -> String {
                format!("{:#x}", self)
            }
        }
    };
}

impl_from_to_hex!(primitive_types::U128);
impl_from_to_hex!(primitive_types::U256);
impl_from_to_hex!(primitive_types::U512);
