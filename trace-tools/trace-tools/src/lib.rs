// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Diagnostics components for async.

#![deny(missing_docs)]

/// Contains [`tracing::Subscriber`] implementation for [`span`](`tracing::Span`) diagnostics.
pub mod subscriber;
/// Contains diagnostic utilities that are separate from [`tracing`].
pub mod util;

mod error;
mod observe;

pub use error::Error;
pub use observe::Observe;

pub use trace_tools_attributes::observe;
