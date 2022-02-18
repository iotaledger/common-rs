// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod flamegraph;
mod log;

pub use self::{flamegraph::FlamegraphLayer, log::LogLayer};

use crate::{util::Flamegrapher, Error};

use fern_logger::LoggerConfig;

use tracing::Metadata;

use std::path::Path;

/// Creates a new [`FlamegraphLayer`].
///
/// The given path describes the desired output location of the folded stack file that is generated by
/// this layer during runtime. This file can then be used to produce a flamegraph by a [`Flamegrapher`]
/// instance, or by the [`inferno`] tool.
///
/// Returns a `Result` of the tuple containing the [`FlamegraphLayer`] and a [`Flamegrapher`] that
/// can be used to produce the flamegraph image at the end of profiling. This can be ignored if you just
/// need the folded stack file, or will use [`inferno`] externally for the graph generation.
///
/// # Errors
/// This function can fail in the following ways:
///  - There was an error creating/truncating the folded stack file at the given location.
///
/// # Panics
/// This function will panic if the program is not built with `--cfg tokio_unstable`.
pub fn flamegraph_layer<P: AsRef<Path>>(
    stack_filename: P,
) -> Result<(FlamegraphLayer, Flamegrapher), Error> {
    #![allow(clippy::assertions_on_constants)]
    assert!(
        cfg!(tokio_unstable),
        "task tracing requires building with RUSTFLAGS=\"--cfg tokio_unstable\"!"
    );

    FlamegraphLayer::new(stack_filename)
}

/// Filter function for the [`FlamegraphLayer`].
///
/// Ignores any [`Event`](tracing::Event)s, and registers [`Span`](tracing::Span)s that have either been
/// instrumented internally by `tokio`, or with [`observe`](trace_tools_attributes::observe).
pub(crate) fn flamegraph_filter(meta: &Metadata<'_>) -> bool {
    if meta.is_event() {
        return false;
    }

    meta.name().starts_with("runtime")
        || meta.target().starts_with("tokio")
        || meta.target() == "trace_tools::observe"
}

/// Creates a new [`LogLayer`], using the parameters provided by the given [`LoggerConfig`].
///
/// This should allow the subscriber to perform logging in an identical fashion to the functionality provided
/// in [`fern_logger`].
///
/// # Errors
/// This function can fail in the following ways:
///  - An [`io::Error`](std::io::Error) was encountered when creating any log files required by the config.
pub fn log_layer(config: LoggerConfig) -> Result<LogLayer, Error> {
    LogLayer::new(config)
}

/// Filter function for the log layer. Registers all [`Event`](tracing::Event)s with the layer.
pub(crate) fn log_filter(meta: &tracing::Metadata<'_>) -> bool {
    meta.is_event()
}

/// Creates a new [`console_subscriber::ConsoleLayer`].
#[cfg(feature = "tokio-console")]
pub fn console_layer() -> Result<console_subscriber::ConsoleLayer, Error> {
    #![allow(clippy::assertions_on_constants)]
    assert!(
        cfg!(tokio_unstable),
        "task tracing requires building with RUSTFLAGS=\"--cfg tokio_unstable\"!"
    );

    let (layer, server) = console_subscriber::ConsoleLayer::builder()
        .with_default_env()
        .build();

    std::thread::Builder::new()
        .name("console_subscriber".into())
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .expect("console subscriber runtime initialization failed");

            runtime.block_on(async move {
                server
                    .serve()
                    .await
                    .expect("console subscriber server failed")
            });
        })
        .expect("console subscriber could not spawn thread");

    Ok(layer)
}

/// Filter function for the console layer.
///
/// Registers relevant [`Event`](tracing::Event)s and [`Span`](tracing::Span)s. This is identical to the
/// filter function used in `tokio`'s `console_subscriber`.
#[cfg(feature = "tokio-console")]
pub(crate) fn console_filter(meta: &tracing::Metadata<'_>) -> bool {
    if meta.is_event() {
        return meta.target().starts_with("runtime") || meta.target().starts_with("tokio");
    }

    meta.name().starts_with("runtime") || meta.target().starts_with("tokio")
}
