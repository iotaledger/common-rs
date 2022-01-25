// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use auth_helper::jwt;

#[test]
fn jwt_valid() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(1000)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::new(claims, b"secret").unwrap();

    assert!(jwt
        .validate(
            String::from("issuer"),
            String::from("subject"),
            String::from("audience"),
            false,
            b"secret",
        )
        .is_ok());
}

#[test]
fn jwt_to_str_from_str_valid() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(1000)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::from(
        jwt::JsonWebToken::new(claims, b"secret")
            .unwrap()
            .to_string(),
    );

    assert!(jwt
        .validate(
            String::from("issuer"),
            String::from("subject"),
            String::from("audience"),
            false,
            b"secret",
        )
        .is_ok());
}

#[test]
fn jwt_invalid_issuer() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(1000)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::new(claims, b"secret").unwrap();

    assert!(jwt
        .validate(
            String::from("Issuer"),
            String::from("subject"),
            String::from("audience"),
            false,
            b"secret",
        )
        .is_err());
}

#[test]
fn jwt_invalid_subject() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(1000)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::new(claims, b"secret").unwrap();

    assert!(jwt
        .validate(
            String::from("issuer"),
            String::from("Subject"),
            String::from("audience"),
            false,
            b"secret",
        )
        .is_err());
}

#[test]
fn jwt_invalid_audience() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(1000)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::new(claims, b"secret").unwrap();

    assert!(jwt
        .validate(
            String::from("issuer"),
            String::from("subject"),
            String::from("Audience"),
            false,
            b"secret",
        )
        .is_err());
}

#[test]
fn jwt_invalid_secret() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(1000)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::new(claims, b"secret").unwrap();

    assert!(jwt
        .validate(
            String::from("issuer"),
            String::from("subject"),
            String::from("audience"),
            false,
            b"Secret",
        )
        .is_err());
}

#[test]
fn jwt_invalid_expired() {
    let claims = jwt::ClaimsBuilder::new(
        String::from("issuer"),
        String::from("subject"),
        String::from("audience"),
    )
    .with_expiry(0)
    .build()
    .unwrap();

    let jwt = jwt::JsonWebToken::new(claims, b"secret").unwrap();

    assert!(jwt
        .validate(
            String::from("issuer"),
            String::from("subject"),
            String::from("audience"),
            true,
            b"secret",
        )
        .is_err());
}
