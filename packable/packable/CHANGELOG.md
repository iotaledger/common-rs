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

## 0.8.3 - 2023-09-15

### Added

- `UnpackOrderedSetError`;

### Fixed

- `BTreeSet` impls and `BTreeSetPrefix` unpack now fails if the data is unordered;

## 0.8.2 - 2023-09-07

### Added

- `BTreeSet` impls and `BTreeSetPrefix`;

## 0.8.1 - 2023-05-02

### Fixed

- `no_std` support;

## 0.8.0 - 2023-03-30

### Changed

- `Debug` impl of `StringPrefix`, `BoxedSlicePrefix` and `VecPrefix` don't print the wrapper anymore;

## 0.7.0 - 2022-10-10

### Changed

- Updated dependencies;

## 0.6.2 - 2022-08-26

### Fixed

- `verify_with` visitor parameter type from `field::UnpackVisitor` to `Self::UnpackVisitor`;

## 0.6.1 - 2022-08-26

### Added

- `verify_with` attribute functions now also take the `visitor` as parameter;

## 0.6.0 - 2022-08-25

### Added

- `Packable::UnpackVisitor` associated type;
- `Packable::{unpack, unpack_verified}` argument `visitor`;
- `#[packable(unpack_visitor = _)]` attribute;

### Changed

- Updated dependencies;

## 0.5.0 - 2022-07-14

### Added

- Add the `SliceUnpacker` type;

### Removed

- Remove the `Unpacker` implementation for `&[u8]`.

## 0.4.0 - 2022-06-13

### Added

- Add the `StringPrefix` type;

## 0.3.3 - 2022-05-27

### Added

- Propagate alternate `Debug` to `VecPrefix` and `BoxedSlicePrefix`;

## 0.3.2 - 2022-05-04

### Added

- Custom `Debug` impl for `VecPrefix`;
- Custom `Debug` impl for `BoxedSlicePrefix`;

## 0.3.1 - 2022-04-20

### Added

- Add `Packer::written_bytes` and `Unpacker::read_bytes`;

## 0.3.0 - 2022-04-19

### Added

- Implement `Packable` for tuples;
- Add `CounterPacker` and `CounterUnpacker`;
- Implement `Packer` and `Unpacker` for mutable references;

### Changed

- Inline more functions in a less aggressive way;
- Make byte buffers packing and unpacking more performant;
- Require `From<Infallible>` for every `Packable::UnpackError`;

### Removed

- Remove `UnpackErrorExt::infallible` as it superseeded by `UnpackErrorExt::coerce`;

## 0.2.1 - 2022-02-11

### Added

- Make the `Error` implementation for `UnknownTagError` less restricted;
- Implement `Error` for `UnpackError`;
- Document the `serde` and `primitive-types` features;

### Changed

- Make `String` packing more performant by using `Packer::pack_bytes` directly;
- Fix documentation of `UnexpectedEOF` and `UnpackPrefixError`;

## 0.2.0 - 2022-02-09

### Added

- Derive `Error` for all the error types if the `std` feature is enabled;
- Implement `Packable` for `f32` and `f64`;
- Implement `Packable` for `usize`, `isize`, `Vec<T>`, `Box<[T]>` and `String` under the `usize` feature;
- Implement `Into<Infallible>` for `UnpackError<Infallible, Infallible>` and `UnpackPrefixError<Infallible, Infallible>`;
- Add the `UnpackError::into_packable_err` and `UnpackPrefixError::into_item_err` methods;

### Changed

- Rename `UnpackError::into_unpacker` to `UnpackError::into_unpacker_err`;
- Rename `UnpackPrefixError::Packable` to `UnpackPrefixError::Item`;
- Rename `UnpackPrefixError::into_prefix` to `UnpackPrefixError::into_prefix_err`;
- Rename `UnpackPrefixError::unwrap_packable_or_else` to `UnpackPrefixError::unwrap_item_err_or_else`;

## 0.1.0 - 2022-01-13

### Added

- Initial features;
