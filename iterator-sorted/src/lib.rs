// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Provides `stable` functions for checking that an `Iterator` is sorted.

#![no_std]
#![deny(missing_docs)]

use core::{cmp::Ordering, iter::Iterator};

/// Checks if an iterator yields ordered and unique values based on a given comparator.
pub fn is_unique_sorted_by<T: Ord, I: Iterator<Item = T>, F: FnMut(&T, &T) -> Ordering>(
    mut iterator: I,
    mut cmp: F,
) -> bool {
    let mut previous = match iterator.next() {
        Some(e) => e,
        None => return true,
    };

    for curr in iterator {
        if cmp(&previous, &curr) != Ordering::Less {
            return false;
        }
        previous = curr;
    }

    true
}

/// Checks if an iterator yields ordered and unique values.
pub fn is_unique_sorted<T: Ord, I: Iterator<Item = T>>(iterator: I) -> bool {
    is_unique_sorted_by(iterator, T::cmp)
}

/// Checks if an iterator yields ordered values based on a given comparator.
pub fn is_sorted_by<T: Ord, I: Iterator<Item = T>, F: FnMut(&T, &T) -> Ordering>(mut iterator: I, mut cmp: F) -> bool {
    let mut previous = match iterator.next() {
        Some(e) => e,
        None => return true,
    };

    for curr in iterator {
        if cmp(&previous, &curr) == Ordering::Greater {
            return false;
        }
        previous = curr;
    }

    true
}

/// Checks if an iterator yields ordered values.
pub fn is_sorted<T: Ord, I: Iterator<Item = T>>(iterator: I) -> bool {
    is_sorted_by(iterator, T::cmp)
}
