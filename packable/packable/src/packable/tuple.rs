// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::borrow::Borrow;

use crate::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

macro_rules! tuple_impls {
    ($($Tuple:ident { ($first_idx:tt) -> $FirstT:ident $(($idx:tt) -> $T:ident)* })+) => {
        $(
            impl<$FirstT: Packable,$($T: Packable),*> Packable for ($FirstT,$($T,)*)
            where
                $($T::UnpackError: Into<$FirstT::UnpackError>,)*
                $($FirstT::UnpackVisitor: Borrow<$T::UnpackVisitor>,)*
            {
                type UnpackError = $FirstT::UnpackError;
                type UnpackVisitor = $FirstT::UnpackVisitor;

                fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
                    self.$first_idx.pack(packer)?;
                    $( self.$idx.pack(packer)?; )*

                    Ok(())
                }

                fn unpack<U: Unpacker>(
                    unpacker: &mut U,
                    visitor: Option<&Self::UnpackVisitor>,
                ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
                    Ok((
                            <$FirstT>::unpack(unpacker, visitor)?,
                            $( (<$T>::unpack_inner(unpacker, visitor).map_packable_err(Into::into))?,)*
                       ))
                }
            }
        )*
    };
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}
