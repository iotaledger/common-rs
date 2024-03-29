// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A module that provides password utilities.

use argon2::{self, Config};
use rand::Rng;

/// Error occurring when hashing/verifying passwords.
pub type Error = argon2::Error;

/// Generates a salt to be used for password hashing.
pub fn generate_salt() -> [u8; 32] {
    rand::rngs::OsRng.gen()
}

/// Hashes a password together with a salt.
pub fn password_hash(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, Error> {
    argon2::hash_raw(password, salt, &Config::default())
}

/// Verifies if a password/salt pair matches a password hash.
pub fn password_verify(password: &[u8], salt: &[u8], hash: &[u8]) -> Result<bool, Error> {
    Ok(hash == password_hash(password, salt)?)
}
