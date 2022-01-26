// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A module that provides JSON Web Token utilities.

pub use jsonwebtoken::TokenData;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::time::{SystemTime, UNIX_EPOCH};

/// JsonWebToken error.
#[derive(Error, Debug)]
pub enum Error {
    /// Provided an invalid expiry date.
    #[error("invalid expiry time {expiry} from issue time {issued_at}")]
    InvalidExpiry { issued_at: u64, expiry: u64 },
    /// An error occured in the [`jsonwebtoken`] crate.
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

/// Represents registered JSON Web Token Claims.
/// <https://tools.ietf.org/html/rfc7519#section-4.1>
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer.
    /// Identifies the principal that issued the JWT. The processing of this claim is generally application specific.
    iss: String,
    /// Subject.
    /// Identifies the principal that is the subject of the JWT. The claims in a JWT are normally statements about the
    /// subject. The subject value MUST either be scoped to be locally unique in the context of the issuer or be
    /// globally unique. The processing of this claim is generally application specific.
    sub: String,
    /// Audience.
    /// Identifies the recipients that the JWT is intended for. Each principal intended to process the JWT MUST
    /// identify itself with a value in the audience claim. If the principal processing the claim does not identify
    /// itself with a value in the "aud" claim when this claim is present, then the JWT MUST be rejected. The
    /// interpretation of audience values is generally application specific.
    aud: String,
    /// Expiration Time.
    /// Identifies the expiration time on or after which the JWT MUST NOT be accepted for processing. The processing of
    /// the "exp" claim requires that the current date/time MUST be before the expiration date/time listed in the "exp"
    /// claim. Implementers MAY provide for some small leeway, usually no more than a few minutes, to account for clock
    /// skew.
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<u64>,
    /// Not Before.
    /// Identifies the time before which the JWT MUST NOT be accepted for processing. The processing of the "nbf" claim
    /// requires that the current date/time MUST be after or equal to the not-before date/time listed in the "nbf"
    /// claim. Implementers MAY provide for some small leeway, usually no more than a few minutes, to account for clock
    /// skew.
    nbf: u64,
    /// Issued At.
    /// Identifies the time at which the JWT was issued. This claim can be used to determine the age of the JWT.
    iat: u64,
}

impl Claims {
    /// Creates a new set of claims.
    fn new(iss: String, sub: String, aud: String, nbf: u64) -> Self {
        Self {
            iss,
            sub,
            aud,
            exp: None,
            nbf,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Clock may have gone backwards")
                .as_secs() as u64,
        }
    }

    /// Returns the issuer of the JWT.
    pub fn issuer(&self) -> &str {
        &self.iss
    }

    /// Returns the subject of the JWT.
    pub fn subject(&self) -> &str {
        &self.sub
    }

    /// Returns the audience of the JWT.
    pub fn audience(&self) -> &str {
        &self.aud
    }

    /// Returns the expiration time of the JWT, if it has been specified.
    pub fn expiry(&self) -> Option<u64> {
        self.exp
    }

    /// Returns the "nbf" field of the JWT.
    pub fn not_before(&self) -> u64 {
        self.nbf
    }

    /// Returns the issue timestamp of the JWT.
    pub fn issued_at(&self) -> u64 {
        self.iat
    }
}

/// Builder for the [`Claims`] structure.
pub struct ClaimsBuilder {
    iss: String,
    sub: String,
    aud: String,
    exp: Option<u64>,
}

impl ClaimsBuilder {
    /// Creates a new [`ClaimsBuilder`] with the given mandatory parameters.
    pub fn new(iss: String, sub: String, aud: String) -> Self {
        Self {
            iss,
            sub,
            aud,
            exp: None,
        }
    }

    /// Specifies that this token will expire, and provides an expiry time (offset from issue time).
    #[must_use]
    pub fn with_expiry(mut self, exp: u64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Builds and returns a [`Claims`] structure using the given builder options.
    pub fn build(self) -> Result<Claims, Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_secs() as u64;

        let mut claims = Claims::new(self.iss, self.sub, self.aud, now);

        if let Some(exp) = self.exp {
            claims.exp = now
                .checked_add(exp)
                .ok_or_else(|| Error::InvalidExpiry {
                    issued_at: now,
                    expiry: exp,
                })
                .map(|exp| Some(exp))?;
        }

        Ok(claims)
    }
}

/// Represents a JSON Web Token.
/// <https://tools.ietf.org/html/rfc7519>
#[derive(Clone, Debug)]
pub struct JsonWebToken(String);

impl From<String> for JsonWebToken {
    fn from(inner: String) -> Self {
        JsonWebToken(inner)
    }
}

impl std::fmt::Display for JsonWebToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JsonWebToken {
    /// Creates a new JSON Web Token.
    pub fn new(claims: Claims, secret: &[u8]) -> Result<Self, Error> {
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )?;

        Ok(Self(token))
    }

    /// Validates a JSON Web Token.
    pub fn validate(
        &self,
        issuer: String,
        subject: String,
        audience: String,
        expires: bool,
        secret: &[u8],
    ) -> Result<TokenData<Claims>, Error> {
        let mut validation = Validation {
            iss: Some(issuer),
            sub: Some(subject),
            validate_exp: expires,
            ..Default::default()
        };
        validation.set_audience(&[audience]);

        Ok(decode::<Claims>(
            &self.0,
            &DecodingKey::from_secret(secret),
            &validation,
        )?)
    }
}
