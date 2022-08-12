# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## Unreleased - YYYY-MM-DD

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security -->

## Unreleased - 2022-XX-XX

### Changed

- Updated dependencies;

## 0.3.0 - 2022-06-14

### Added

 - `BuildValidation` extension trait for the `jsonwebtoken::Validation` type;
 - `Claims` builder-lite methods;

### Changed

 - Re-export `jsonwebtoken` lib;
 - Expose `Claims` and `JsonWebToken` inner fields;
 - `Claims` system time error handling;
 - Better test cases;
 - `JsonWebToken::validate` accepts `Validation` struct for better flexibility;
 
### Removed

 - `ClaimsBuilder`;

## 0.2.0 - 2022-01-25

### Added

 - Capability to create tokens that do not expire;
 - `jwt::Error` type;
 - `ClaimsBuilder` for optional claims;

### Changed

 - `JsonWebToken` interface changes;

## 0.1.0 - 2022-01-20

### Added

- Initial features;
