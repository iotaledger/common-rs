// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use tracing::field::{Field, Visit};

use crate::observe::{FILE_FIELD_NAME, LINE_FIELD_NAME};

/// Visits a [`Span`](tracing::Span) and records location fields.
///
/// In `tokio`'s internal instrumentation, these fields are identified with the keys `"loc.line"`,
/// `"loc.file"`, and `"loc.col"`. Currently, the column field is ignored by this visitor.
#[derive(Default)]
pub(crate) struct LocationVisitor {
    /// The file of the [`Span`](tracing::Span) location.
    pub(crate) file: Option<String>,
    /// The line of the [`Span`](tracing::Span) loaction.
    pub(crate) line: Option<u32>,
}

impl LocationVisitor {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Visit for LocationVisitor {
    fn record_debug(&mut self, _field: &Field, _value: &dyn fmt::Debug) {}

    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == LINE_FIELD_NAME {
            self.line = Some(value as u32);
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == FILE_FIELD_NAME {
            self.file = Some(value.to_string());
        }
    }
}

/// Visits a [`Span`](tracing::Span) and records message fields.
///
/// The `message` field is very common in [`tracing`] spans and events, and is also used by events
/// generated by [`tracing_log`] to describe [`log`] event messages.
#[derive(Default)]
pub(crate) struct MessageVisitor(pub(crate) String);

impl MessageVisitor {
    /// The field name that describes an event message.
    const FIELD_NAME: &'static str = "message";
}

impl Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == Self::FIELD_NAME {
            self.0 = format!("{:?}", value);
        }
    }
}
