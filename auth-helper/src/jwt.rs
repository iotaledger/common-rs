// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A module that provides JSON Web Token utilities.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use jsonwebtoken::{self, Validation};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// JsonWebToken error.
#[derive(Error, Debug)]
pub enum Error {
    /// Provided an invalid expiry date.
    #[error("invalid expiry time {expiry} from issue time {issued_at}")]
    InvalidExpiry { issued_at: u64, expiry: u64 },
    /// Provided an invalid not before date.
    #[error("invalid not before time {nbf} from issue time {issued_at}")]
    InvalidNbf { issued_at: u64, nbf: u64 },
    /// The system time is before the UNIX epoch.
    #[error("system time is before the UNIX epoch")]
    InvalidSystemTime,
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
    pub iss: String,
    /// Subject.
    /// Identifies the principal that is the subject of the JWT. The claims in a JWT are normally statements about the
    /// subject. The subject value MUST either be scoped to be locally unique in the context of the issuer or be
    /// globally unique. The processing of this claim is generally application specific.
    pub sub: String,
    /// Audience.
    /// Identifies the recipients that the JWT is intended for. Each principal intended to process the JWT MUST
    /// identify itself with a value in the audience claim. If the principal processing the claim does not identify
    /// itself with a value in the "aud" claim when this claim is present, then the JWT MUST be rejected. The
    /// interpretation of audience values is generally application specific.
    pub aud: String,
    /// Expiration Time.
    /// Identifies the expiration time on or after which the JWT MUST NOT be accepted for processing. The processing of
    /// the "exp" claim requires that the current date/time MUST be before the expiration date/time listed in the "exp"
    /// claim. Implementers MAY provide for some small leeway, usually no more than a few minutes, to account for clock
    /// skew.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    /// Not Before.
    /// Identifies the time before which the JWT MUST NOT be accepted for processing. The processing of the "nbf" claim
    /// requires that the current date/time MUST be after or equal to the not-before date/time listed in the "nbf"
    /// claim. Implementers MAY provide for some small leeway, usually no more than a few minutes, to account for clock
    /// skew.
    pub nbf: u64,
    /// Issued At.
    /// Identifies the time at which the JWT was issued. This claim can be used to determine the age of the JWT.
    pub iat: u64,
}

impl Claims {
    /// Creates a new set of claims.
    pub fn new(iss: impl Into<String>, sub: impl Into<String>, aud: impl Into<String>) -> Result<Self, Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::InvalidSystemTime)?
            .as_secs();
        Ok(Self {
            iss: iss.into(),
            sub: sub.into(),
            aud: aud.into(),
            exp: None,
            nbf: now,
            iat: now,
        })
    }

    /// Specify that this token will expire by providing an expiry timestamp.
    pub fn expires_after(mut self, exp: u64) -> Result<Self, Error> {
        if exp < self.iat {
            return Err(Error::InvalidExpiry {
                issued_at: self.iat,
                expiry: exp,
            });
        }
        self.exp = Some(exp);
        Ok(self)
    }

    /// Specify that this token will expire by providing a duration offset from issued time.
    pub fn expires_after_duration(mut self, dur: Duration) -> Result<Self, Error> {
        let dur = dur.as_secs();
        let exp = self.iat.checked_add(dur).ok_or(Error::InvalidExpiry {
            issued_at: self.iat,
            expiry: self.iat + dur,
        })?;
        self.exp = Some(exp);
        Ok(self)
    }

    /// Specify that this token is valid after the given timestamp.
    pub fn valid_after(mut self, nbf: u64) -> Result<Self, Error> {
        if nbf < self.iat {
            return Err(Error::InvalidNbf {
                issued_at: self.iat,
                nbf,
            });
        }
        self.nbf = nbf;
        Ok(self)
    }

    /// Specify when this token becomes valid by providing a duration offset from issued time.
    pub fn valid_after_duration(mut self, dur: Duration) -> Result<Self, Error> {
        let dur = dur.as_secs();
        let nbf = self.iat.checked_add(dur).ok_or(Error::InvalidNbf {
            issued_at: self.iat,
            nbf: self.iat + dur,
        })?;
        self.nbf = nbf;
        Ok(self)
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

pub trait BuildValidation: _sealed_validation::SealedBuildValidation {
    fn with_audience(self, aud: impl ToString) -> Self;

    fn with_audiences(self, auds: &[impl ToString]) -> Self;

    fn with_issuer(self, iss: impl ToString) -> Self;

    fn with_issuers(self, iss: &[impl ToString]) -> Self;

    fn with_subject(self, sub: impl ToString) -> Self;

    fn with_required_spec_claims(self, claims: &[&str]) -> Self;

    fn with_leeway(self, secs: u64) -> Self;

    fn validate_exp(self, validate: bool) -> Self;

    fn validate_nbf(self, validate: bool) -> Self;
}

impl BuildValidation for Validation {
    fn with_audience(mut self, aud: impl ToString) -> Self {
        self.set_audience(&[aud]);
        self
    }

    fn with_audiences(mut self, auds: &[impl ToString]) -> Self {
        self.set_audience(auds);
        self
    }

    fn with_issuer(mut self, iss: impl ToString) -> Self {
        self.set_issuer(&[iss]);
        self
    }

    fn with_issuers(mut self, iss: &[impl ToString]) -> Self {
        self.set_issuer(iss);
        self
    }

    fn with_subject(mut self, sub: impl ToString) -> Self {
        self.sub = Some(sub.to_string());
        self
    }

    fn with_required_spec_claims(mut self, claims: &[&str]) -> Self {
        self.set_required_spec_claims(claims);
        self
    }

    fn with_leeway(mut self, secs: u64) -> Self {
        self.leeway = secs;
        self
    }

    fn validate_exp(mut self, validate: bool) -> Self {
        self.validate_exp = validate;
        self
    }

    fn validate_nbf(mut self, validate: bool) -> Self {
        self.validate_nbf = validate;
        self
    }
}

mod _sealed_validation {
    pub trait SealedBuildValidation {}
    impl SealedBuildValidation for jsonwebtoken::Validation {}
}

/// Represents a JSON Web Token.
/// <https://tools.ietf.org/html/rfc7519>
#[derive(Clone, Debug)]
pub struct JsonWebToken(pub String);

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
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))?;

        Ok(Self(token))
    }

    /// Validates a JSON Web Token.
    pub fn validate(&self, validation: impl Into<Validation>, secret: &[u8]) -> Result<TokenData<Claims>, Error> {
        Ok(decode::<Claims>(
            &self.0,
            &DecodingKey::from_secret(secret),
            &validation.into(),
        )?)
    }
}
