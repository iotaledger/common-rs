// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;

use hex::FromHexError;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPrefix { c0: char, c1: char },
    InvalidHexCharacter { c: char, index: usize },
    InvalidStringLength,
    InvalidStringLengthSlice { expected: usize, actual: usize },
    OddLength,
}

impl Into<Error> for FromHexError {
    fn into(self) -> Error {
        match self {
            Self::InvalidHexCharacter { c, index } => Error::InvalidHexCharacter { c, index },
            Self::InvalidStringLength => Error::InvalidStringLength,
            Self::OddLength => Error::OddLength,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidPrefix { c0, c1 } => {
                write!(f, "Invalid prefix `{c0}{c1}`, should be `0x`")
            }
            Error::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character {:?} at position {}", c, index)
            }
            Error::InvalidStringLength => write!(f, "Invalid string length"),
            Error::InvalidStringLengthSlice { expected, actual } => write!(
                f,
                "invalid hexadecimal length for slice: expected {expected} got {actual}"
            ),
            Error::OddLength => write!(f, "Odd number of digits in hex string"),
        }
    }
}
