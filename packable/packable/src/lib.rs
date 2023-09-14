// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Packable is a binary serialization and deserialization framework.
//!
//! # Design
//!
//! Values of a type can be serialized and deserialized if the type implements the [`Packable`]
//! trait. The serialization strategy used for each type is up to the user. However, [`Packable`]
//! can also be derived, this provides a consistent serialization strategy.
//!
//! For more information about the design of this crate please read the [`Packable`], [`unpacker`],
//! [`packer`], [`UnpackError`](error::UnpackError) and [`UnpackErrorExt`](error::UnpackErrorExt)
//! documentation.
//!
//! # `no_std` compatibility
//!
//! Packable is `no_std` compatible. This is achieved by introducing the [`Packer`](packer::Packer)
//! and [`Unpacker`](unpacker::Unpacker) traits to abstract away any IO operation without relying on
//! [`std::io`]. This has the additional benefit of allowing us to pack and unpack values from
//! different kinds of buffers.
//!
//! # Types that implement [`Packable`]
//!
//! The [`Packable`] trait is implemented for every sized integer type by encoding the value as an
//! array of bytes in little-endian order.
//!
//! Booleans are packed following Rust's data layout, meaning that `true` is packed as a `1` byte
//! and `false` as a `0` byte. However, boolean unpacking is less strict and unpacks any non-zero
//! byte as `true`.
//!
//! Types such as `Box<[T]>`, `[T; N]` and [`Option<T>`] implement [`Packable`] if `T` implements
//! [`Packable`].
//!
//! This crate also provides bounded integers under the [`mod@bounded`] module which have additional
//! syntactical checks to guarantee that the deserialized values are in-bounds. It is also possible
//! to serialize and deserialize sequences of values by using the types provided in the [`prefix`]
//! module, which represent linear sequences of values with a length prefix.
//!
//! Check the [`Packable`] `impl` section for further information.
//!
//! # Features
//!
//! ## `io`
//!
//! This feature provides the types [`IoPacker`](packer::IoPacker) and
//! [`IoUnpacker`](unpacker::IoUnpacker) which allow packing and unpacking from values whose types
//! implement [`Write`](std::io::Write) and [`Read`](std::io::Read) respectively.
//!
//! ## `primitive-types`
//!
//! This feature implements [`Packable`] for [`U256`](primitive_types::U256) encoding its values as
//! arrays of bytes in little-endian order.
//!
//! ## `serde`
//!
//! This feature derives [`Serialize`](serde::Serialize) and [`Deserialize`](serde::Deserialize)
//! for the types provided in the [`mod@bounded`] and [`prefix`] modules
//!
//! ## `std`
//!
//! This feature implements [`Error`](std::error::Error) for all the error types provided by this
//! crate.
//!
//! ## `usize`
//!
//! This feature implements [`Packable`] for [`usize`], [`isize`], [`Vec<T>`](std::vec::Vec),
//! `Box<[T]>` and [`String`]. This is done serializing and deserializing
//! pointer sized integers as 64-bit integers. This feature will not work for targets with a
//! pointer width larger than 64.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

#[cfg(any(doc, feature = "std"))]
extern crate std;

mod packable;

pub mod error;
pub mod packer;
pub mod unpacker;

pub use crate::packable::*;
