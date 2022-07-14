// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A module that provides a [`Packable`] trait to serialize and deserialize types.

extern crate alloc;

pub mod bounded;
pub mod option;
pub mod prefix;

mod array;
mod bool;
mod r#box;
mod num;
#[cfg(feature = "primitive-types")]
mod primitive_types;
#[cfg(feature = "usize")]
mod string;
mod tuple;
#[cfg(feature = "usize")]
mod vec;

use alloc::vec::Vec;
use core::{
    convert::{AsRef, Infallible},
    fmt::Debug,
};

pub use packable_derive::Packable;

use crate::{
    error::{UnexpectedEOF, UnpackError},
    packer::{LenPacker, Packer},
    unpacker::{SliceUnpacker, Unpacker},
};

/// A type that can be packed and unpacked.
///
/// Almost all basic sized types implement this trait. This trait can be derived using the
/// [`Packable`](packable_derive::Packable) macro. The following example shows how to implement this trait manually.
///
/// # Example
///
/// We will implement [`Packable`] for a type that encapsulates optional integer values (like `Option<i32>`).
///
/// We will use an integer prefix as a tag to determine which variant of the enum is being packed.
///
/// ```rust
/// use core::convert::Infallible;
///
/// use packable::{
///     error::{UnknownTagError, UnpackError, UnpackErrorExt},
///     packer::Packer,
///     unpacker::Unpacker,
///     Packable,
/// };
///
/// pub enum Maybe {
///     Nothing,
///     Just(i32),
/// }
///
/// impl Packable for Maybe {
///     type UnpackError = UnknownTagError<u8>;
///
///     fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
///         match self {
///             // Pack a `0` byte and nothing else.
///             Self::Nothing => 0u8.pack(packer),
///             // Pack a `1` byte followed by the internal value.
///             Self::Just(value) => {
///                 1u8.pack(packer)?;
///                 value.pack(packer)
///             }
///         }
///     }
///
///     fn unpack<U: Unpacker, const VERIFY: bool>(
///         unpacker: &mut U,
///     ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
///         match u8::unpack::<_, VERIFY>(unpacker).coerce()? {
///             0u8 => Ok(Self::Nothing),
///             1u8 => Ok(Self::Just(i32::unpack::<_, VERIFY>(unpacker).coerce()?)),
///             tag => Err(UnpackError::Packable(UnknownTagError(tag))),
///         }
///     }
/// }
/// ```
/// To understand the behavior of `infallible` and `coerce` check the [`UnpackError`] and
/// [`UnpackErrorExt`](crate::error::UnpackErrorExt) documentation.
///
/// We can also derive `Packable` for the `Maybe` type.
/// ```rust
/// # use packable as packable_crate;
/// use packable::Packable;
///
/// #[derive(Packable)]
/// #[packable(tag_type = u8)]
/// pub enum Maybe {
///     #[packable(tag = 0)]
///     Nothing,
///     #[packable(tag = 1)]
///     Just(i32),
/// }
/// ```
/// The code produced by this macro is equivalent to the one shown before.
///
/// # `#[derive(Packable)]` attributes
///
/// The derive implementation can be tweaked using `#[packable(...)]` attributes.
///
/// ## Tags for enums
/// A very common pattern when implementing `Packable` for enums consists in introducing a prefix
/// value to differentiate each variant of the enumeration when unpacking, this prefix value is
/// known as a `tag`. The type of the `tag` is specified with the `#[packable(tag_type = ...)]`
/// attribute and it can only be one of `[u8]`, `[u16]`, `[u32]` or `[u64]`. The `tag` value used
/// for each variant is specified with the `#[packable(tag = ...)]` attribute and can only contain
/// integer literal without any type prefixes (e.g. `42` is valid but `42u8` is not).
///
/// In the example above, the `tag` type is `[u8]`, the `Nothing` variant has a `tag` value of `0`
/// and the `Just` variant has a `tag` value of `1`. This means that the packed version of
/// `Maybe::Nothing` is `[0]` and the packed version of `Maybe::Just(7)` is `[1, 0, 0, 0, 7]`.
///
/// The `tag_type` and `tag` attributes are mandatory for enums unless the enum has a
/// `#[repr(...)]` attribute identifier, in which case the `repr` type will be used as the
/// `tag_type` and each variant discriminant will be used as the `tag`. The `tag_type` and `tag`
/// attributes take precedence over the `repr` attribute.
///
/// ## The `UnpackError` associated type
///
/// The derive macro provides the optional attribute and `#[packable(unpack_error = ...)]` to
/// specify the `UnpackError` associated type. The macro also provides sensible defaults for cases
/// when the attribute is not used.
///
/// For structs, the default [`UnpackError`](Packable::UnpackError) type is the
/// [`UnpackError`](Packable::UnpackError) of any of the fields type or
/// [`Infallible`](core::convert::Infallible) in case the struct has no fields.
///
/// For enums, the default  [`UnpackError`](Packable::UnpackError) type is
/// [`UnknownTagError<T>`](crate::error::UnknownTagError) where `T` is the type specified according
/// to the `tag_type` or `repr` attributes.
///
/// Following the example above, `Maybe::UnpackError` is `UnknownTagError<u8>` because no
/// `unpack_error` attribute was specified.
///
/// ## Error conversion
///
/// The `unpack_error` attribute can also receive an optional additional argument using the `with`
/// identifier: `#[packable(unpack_error = ..., with = ...)]`. This `with` argument must be a Rust
/// expression and it is used to map the `UnpackError` produced while unpacking each one of the
/// fields of the type.
///
/// Sometimes it is required to map the `UnpackError` for each field individually. The
/// `#[packable(unpack_error_with = ...)]` attribute can be applied to each field for this purpose.
/// This attribute takes precedence over the `with` expression specified in the `unpack_error`
/// attribute.
///
/// The error produced when an invalid `tag` is found while unpacking an `enum` can also be
/// specified using the `with_error` optional argument for the `tag_type` attribute:
/// `#[packable(tag_type = ..., with_error = ...)]`. This argument must be a valid Rust expression.
///
/// ## Additional semantic verifications
///
/// From time to time it is required to do additional semantic verifications over one of more
/// fields of a `struct` or an `enum`'s variant. This can be done using the
/// `#[packable(verify_with = ...)]` attribute which must receive a valid Rust path refering to a
/// function with the signature
/// ```ignore
/// fn<const VERIFY: bool>(field: &F) -> Result<(), P::UnpackError>
/// ```
/// where `F` is the type of the field being verified, `P` is the type of the `struct` or `enum`
/// and `VERIFY` is the same constant parameter used inside `Packable::unpack`. This verification
/// function will be run immediately after unpacking the field.
pub trait Packable: Sized + 'static {
    /// The error type that can be returned if some semantic error occurs while unpacking.
    ///
    /// It is recommended to use [`Infallible`](core::convert::Infallible) if this kind of error is impossible or
    /// [`UnknownTagError`](crate::error::UnknownTagError) when implementing this trait for an enum.
    type UnpackError: Debug + From<Infallible>;

    /// Packs this value into the given [`Packer`].
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error>;

    /// Unpacks this value from the given [`Unpacker`]. The `VERIFY` generic parameter can be used to skip additional
    /// syntactic checks.
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>>;
}

/// Extension trait for types that implement [`Packable`].
pub trait PackableExt: Packable {
    /// Returns the length in bytes of the value after being packed. The returned value always matches the number of
    /// bytes written using `pack`.
    fn packed_len(&self) -> usize;

    /// Convenience method that packs this value into a [`Vec<u8>`].
    fn pack_to_vec(&self) -> Vec<u8>;

    /// Unpacks this value from a sequence of bytes doing syntactical checks.
    fn unpack_verified<T: AsRef<[u8]>>(
        bytes: T,
    ) -> Result<Self, UnpackError<<Self as Packable>::UnpackError, UnexpectedEOF>>;

    /// Unpacks this value from a sequence of bytes without doing syntactical checks.
    fn unpack_unverified<T: AsRef<[u8]>>(
        bytes: T,
    ) -> Result<Self, UnpackError<<Self as Packable>::UnpackError, UnexpectedEOF>>;
}

impl<P: Packable> PackableExt for P {
    #[inline]
    fn packed_len(&self) -> usize {
        let mut packer = LenPacker(0);

        match self.pack(&mut packer) {
            Ok(_) => packer.0,
            Err(e) => match e {},
        }
    }

    #[inline]
    fn pack_to_vec(&self) -> Vec<u8> {
        let mut packer = Vec::with_capacity(self.packed_len());

        // Packing to a `VecPacker` cannot fail.
        self.pack(&mut packer).unwrap();

        packer
    }

    /// Unpacks this value from a type that implements [`AsRef<[u8]>`].
    #[inline]
    fn unpack_verified<T: AsRef<[u8]>>(
        bytes: T,
    ) -> Result<Self, UnpackError<<Self as Packable>::UnpackError, UnexpectedEOF>> {
        Self::unpack::<_, true>(&mut SliceUnpacker::new(bytes.as_ref()))
    }

    /// Unpacks this value from a type that implements [`AsRef<[u8]>`] skipping some syntatical checks.
    #[inline]
    fn unpack_unverified<T: AsRef<[u8]>>(
        bytes: T,
    ) -> Result<Self, UnpackError<<Self as Packable>::UnpackError, UnexpectedEOF>> {
        Self::unpack::<_, false>(&mut SliceUnpacker::new(bytes.as_ref()))
    }
}
