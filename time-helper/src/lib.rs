// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Provides common functionality for handling timestamps.

/// Retrieves the current timestamp, at UTC.
pub fn now_utc() -> time::OffsetDateTime {
    time::OffsetDateTime::now_utc()
}

/// Creates a new time from a unix timestamp, at UTC.
pub fn from_unix_timestamp(timestamp: i64) -> time::OffsetDateTime {
    time::OffsetDateTime::from_unix_timestamp(timestamp).expect("timestamp out of range")
}

/// Produces a formatted `String` from a timestamp, displayed as local time.
pub fn format(time: &time::OffsetDateTime) -> String {
    // This format string is correct, so unwrapping is fine.
    let format_description =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] (UTC)").unwrap();

    // We know this is correct.
    time.format(&format_description).unwrap()
}

/// Takes a unix timestamp and returns a formatted `String`.
pub fn format_unix_timestamp(timestamp: i64) -> String {
    format(&from_unix_timestamp(timestamp))
}
