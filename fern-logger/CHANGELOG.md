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

## 0.4.0 - 2022-02-18

### Added

- Derive `Default` on `LoggerConfig`;

## 0.3.0 - 2022-02-09

### Added

- `serde` aliases to `LoggerOutputConfigBuilder` and `LoggerConfigBuilder` fields;
- Derive `PartialEq` on `LoggerOutputConfig` and `LoggerConfig`;

## 0.2.0 - 2022-01-26

### Added

- Set target exclusions in `LoggerOutputConfigBuilder`;
- Accessors for `LoggerOutputConfig` fields;

## 0.1.0 - 2022-01-18

### Added

- Initial features;
