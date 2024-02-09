// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::string::{FromUtf8Error, String};
use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, Range},
};

use crate::{
    bounded::Bounded,
    error::{UnpackError, UnpackErrorExt},
    packable::Packable,
    packer::Packer,
    prefix::UnpackPrefixError,
    unpacker::Unpacker,
};

/// Wrapper type for [`String`] with a length prefix.
/// The [`String`]'s prefix bounds are provided by `B`, where `B` is a [`Bounded`] type. The prefix
/// type is the `Bounds` type associated with `B`.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct StringPrefix<B: Bounded> {
    inner: String,
    bounded: PhantomData<B>,
}

impl<B: Bounded> fmt::Debug for StringPrefix<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.inner)
        } else {
            write!(f, "{:?}", self.inner)
        }
    }
}

impl<B: Bounded> Default for StringPrefix<B> {
    fn default() -> Self {
        Self {
            inner: String::new(),
            bounded: PhantomData,
        }
    }
}

impl<B: Bounded> Deref for StringPrefix<B> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<B: Bounded> From<StringPrefix<B>> for String {
    fn from(prefix: StringPrefix<B>) -> String {
        prefix.inner
    }
}

impl<B> TryFrom<String> for StringPrefix<B>
where
    B: Bounded,
{
    type Error = <B as TryFrom<usize>>::Error;

    fn try_from(vec: String) -> Result<Self, Self::Error> {
        B::try_from(vec.len())?;

        Ok(Self {
            inner: vec,
            bounded: PhantomData,
        })
    }
}

impl<B> Packable for StringPrefix<B>
where
    B: Bounded + Packable<UnpackVisitor = ()>,
    <B::Bounds as TryInto<B>>::Error: fmt::Debug,
    <B as TryFrom<usize>>::Error: fmt::Debug,
    Range<B::Bounds>: Iterator<Item = B::Bounds>,
{
    type UnpackError = UnpackPrefixError<FromUtf8Error, B::UnpackError>;
    type UnpackVisitor = ();

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        // The length of any dynamically-sized sequence must be prefixed. This unwrap is fine since
        // the length of the inner `String` has been validated while creating this `StringPrefix`.
        B::try_from(self.len()).unwrap().pack(packer)?;

        packer.pack_bytes(self.inner.as_bytes())?;

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        // The length of any dynamically-sized sequence must be prefixed.
        let len = B::unpack(unpacker, visitor)
            .map_packable_err(UnpackPrefixError::Prefix)?
            .into();

        // If `len` does not fit in a `usize`, we panic. There is no way this sequence will fit in memory anyway.
        let len = len
            .try_into()
            .ok()
            .expect("the length prefix exceeds the pointer length of this platform");

        let mut bytes = alloc::vec![0u8; len];
        unpacker.unpack_bytes(&mut bytes)?;

        let inner = String::from_utf8(bytes).map_err(|e| UnpackError::Packable(UnpackPrefixError::Item(e)))?;

        Ok(Self {
            inner,
            bounded: PhantomData,
        })
    }
}
