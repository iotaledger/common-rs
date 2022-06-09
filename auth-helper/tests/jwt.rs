// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use auth_helper::jwt::{BuildValidation, Claims, Error, JsonWebToken, Validation};

#[test]
fn jwt_valid() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    assert!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("audience")
                .with_subject("subject")
                .validate_nbf(true),
            b"secret"
        )
        .is_ok()
    );
}

#[test]
fn jwt_to_str_from_str_valid() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();

    let jwt = JsonWebToken::from(JsonWebToken::new(claims, b"secret").unwrap().to_string());

    assert!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("audience")
                .with_subject("subject")
                .validate_nbf(true),
            b"secret"
        )
        .is_ok()
    );
}

#[test]
fn jwt_invalid_issuer() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    assert!(matches!(
        jwt.validate(
            Validation::default()
                .with_issuer("Issuer")
                .with_audience("audience")
                .with_subject("subject")
                .validate_nbf(true),
            b"secret",
        ),
        Err(Error::Jwt(e)) if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::InvalidIssuer)
    ))
}

#[test]
fn jwt_invalid_subject() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    assert!(matches!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("audience")
                .with_subject("Subject")
                .validate_nbf(true),
            b"secret",
        ),
        Err(Error::Jwt(e)) if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::InvalidSubject)
    ))
}

#[test]
fn jwt_invalid_audience() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    assert!(matches!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("Audience")
                .with_subject("subject")
                .validate_nbf(true),
            b"secret",
        ),
        Err(Error::Jwt(e)) if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::InvalidAudience)
    ))
}

#[test]
fn jwt_invalid_secret() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    assert!(matches!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("audience")
                .with_subject("subject")
                .validate_nbf(true),
            b"Secret",
        ),
        Err(Error::Jwt(e)) if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::InvalidSignature)
    ))
}

#[test]
fn jwt_invalid_expired() {
    let claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(0))
        .unwrap();

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    assert!(matches!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("audience")
                .with_subject("subject")
                .validate_nbf(true)
                .with_leeway(0),
            b"secret",
        ),
        Err(Error::Jwt(e)) if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::ExpiredSignature)
    ))
}

#[test]
fn jwt_immature_signature() {
    let mut claims = Claims::new("issuer", "subject", "audience")
        .unwrap()
        .expires_after_duration(Duration::from_secs(1000))
        .unwrap();
    claims.nbf += 100;

    let jwt = JsonWebToken::new(claims, b"secret").unwrap();

    assert!(matches!(
        jwt.validate(
            Validation::default()
                .with_issuer("issuer")
                .with_audience("audience")
                .with_subject("subject")
                .validate_nbf(true),
            b"secret",
        ),
        Err(Error::Jwt(e)) if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::ImmatureSignature)
    ))
}
