// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Contains layers used in this crate's subscriber for node diagnostics.
pub mod layer;

/// Contains visitors that record [`Span`](tracing::Span) field information.
pub(crate) mod visitors;

use std::path::{Path, PathBuf};

use fern_logger::LoggerConfig;
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::{FilterFn, Filtered},
    layer::Layered,
    prelude::*,
    Registry,
};

use crate::{util::Flamegrapher, Error};

/// Initialises a [`tracing_log::LogTracer`] that converts any incoming [`log`] records into [`tracing`] events,
/// allowing subscribers to interact with log records.
///
/// This is necessary to perform logging through the [`log`] crate whilst also setting the global logger to a
/// [`tracing::Subscriber`] implementation.
///
/// # Errors
/// - Returns an [`Error`] if this function has failed to set the global logger.
///
/// # Notes
///  - This should only be called once. Any subsequent calls will fail, since the global logger can only be set
/// once in a program's lifespan.
///  - If the global logger has already been set (by the [`log`] crate, for example), this will fail.
pub fn collect_logs() -> Result<(), log::SetLoggerError> {
    LogTracer::init()
}

type BaseSubscriber = Layered<
    Filtered<
        Option<layer::LogLayer>,
        FilterFn,
        Layered<Filtered<Option<layer::FlamegraphLayer>, FilterFn, Registry>, Registry>,
    >,
    Layered<Filtered<Option<layer::FlamegraphLayer>, FilterFn, Registry>, Registry>,
>;

/// [`Layered`](tracing_subscriber::layer::Layered) type describing the composition of the subscriber constructed
/// by the [`trace_tools`](crate) crate.
#[cfg(not(feature = "tokio-console"))]
pub type TraceSubscriber = BaseSubscriber;

/// [`Layered`] type describing the composition of the subscriber constructed
/// by the [`trace_tools`](crate) crate.
#[cfg(feature = "tokio-console")]
pub type TraceSubscriber =
    Layered<Filtered<Option<console_subscriber::ConsoleLayer>, FilterFn, BaseSubscriber>, BaseSubscriber>;

/// Builder for the [`trace_tools`](crate) subscriber.
///
/// This can be used to enable/disable [`Layer`](tracing_subscriber::Layer)s provided by this crate.
#[derive(Default)]
#[must_use]
pub struct SubscriberBuilder {
    #[cfg(feature = "tokio-console")]
    console_enabled: bool,

    logger_config: Option<LoggerConfig>,
    flamegraph_stack_file: Option<PathBuf>,
}

impl SubscriberBuilder {
    /// Enables the [`LogLayer`](layer::LogLayer) for this subscriber, using the parameters provided by
    /// the given [`LoggerConfig`].
    pub fn with_log_layer(mut self, logger_config: LoggerConfig) -> Self {
        self.logger_config = Some(logger_config);
        self
    }

    /// Enables the [`LogLayer`](layer::LogLayer) for this subscriber, using the default configuration.
    pub fn with_default_log_layer(mut self) -> Self {
        self.logger_config = Some(LoggerConfig::default());
        self
    }

    /// Enables the [`FlamegraphLayer`](layer::FlamegraphLayer) for this subscriber.
    ///
    /// The given path describes the desired output location of the folded stack file that is generated by
    /// this layer during runtime. This file can then be used to produce a flamegraph by a [`Flamegrapher`]
    /// instance, or by the [`inferno`] tool.
    pub fn with_flamegraph_layer<P: AsRef<Path>>(mut self, folded_stack_file: P) -> Self {
        self.flamegraph_stack_file = Some(folded_stack_file.as_ref().to_path_buf());
        self
    }

    /// Enables the [`console_subscriber::ConsoleLayer`] for this subscriber.
    #[cfg(feature = "tokio-console")]
    pub fn with_console_layer(mut self) -> Self {
        self.console_enabled = true;
        self
    }

    /// Builds and returns the [`TraceSubscriber`].
    ///
    /// # Errors
    ///  - Creation of the [`FlamegraphLayer`](layer::FlamegraphLayer) failed.
    ///  - Creation of the [`LogLayer`](layer::LogLayer) failed.
    ///
    /// # Notes
    ///  - This method calls the [`collect_logs`] function. Any [`log`] records emitted will be converting
    /// into [`tracing`] events, and therefore any external functionality that deals with [`log`] records
    /// may no longer function as expected.
    ///  - This method does *not* set the global subscriber. As such, a call to `finish` can be used to
    /// further extend the return subscriber with external [`Layer`](tracing_subscriber::Layer)s.
    pub fn finish(self) -> Result<(TraceSubscriber, Option<Flamegrapher>), Error> {
        self.compose()
    }

    /// Builds the [`TraceSubscriber`] and sets it as the global default subscriber.
    ///
    /// Returns a `Result` over an [`Option<Flamegrapher>`](Flamegrapher). The returned option is `Some` if
    /// the [`LogLayer`](layer::LogLayer) is enabled and has been successfully initialised. If the
    /// [`LogLayer](layer::LogLayer) has not been enabled with the builder, it is fine to ignore this value.
    ///
    /// # Errors
    ///  - Creation of the [`FlamegraphLayer`](layer::FlamegraphLayer) failed.
    ///  - Creation of the [`LogLayer`](layer::LogLayer) failed.
    ///
    /// # Panics
    /// This method will panic if the flamegraph layer is enabled and the program is not built with
    /// `--cfg tokio_unstable`.
    ///
    /// # Notes
    ///  - This method calls the [`collect_logs`] function. Any [`log`] records emitted will be converting
    /// into [`tracing`] events, and therefore any external functionality that deals with [`log`] records
    /// may no longer function as expected.
    ///  - This method sets the global subscriber. Any further attempts to set the global subscriber
    /// (including another call to this method) will fail.
    ///  - The subscriber initialised by this method cannot be extended.
    pub fn init(self) -> Result<Option<Flamegrapher>, Error> {
        let (subscriber, flamegrapher) = self.compose()?;

        subscriber.init();

        Ok(flamegrapher)
    }

    fn compose(mut self) -> Result<(TraceSubscriber, Option<Flamegrapher>), Error> {
        let (flamegraph_layer, flamegrapher) = self.build_flamegraph_layer()?;
        let log_layer = self.build_log_layer()?;

        let subscriber = tracing_subscriber::registry()
            .with(flamegraph_layer.with_filter(FilterFn::new(
                layer::flamegraph_filter as for<'r, 's> fn(&'r tracing::Metadata<'s>) -> bool,
            )))
            .with(log_layer.with_filter(FilterFn::new(
                layer::log_filter as for<'r, 's> fn(&'r tracing::Metadata<'s>) -> bool,
            )));

        #[cfg(feature = "tokio-console")]
        {
            let console_layer = if self.console_enabled {
                Some(layer::console_layer()?)
            } else {
                None
            };

            let subscriber = subscriber.with(console_layer.with_filter(FilterFn::new(
                layer::console_filter as for<'r, 's> fn(&'r tracing::Metadata<'s>) -> bool,
            )));

            Ok((subscriber, flamegrapher))
        }

        #[cfg(not(feature = "tokio-console"))]
        Ok((subscriber, flamegrapher))
    }

    fn build_log_layer(&mut self) -> Result<Option<layer::LogLayer>, Error> {
        if self.logger_config.is_some() {
            collect_logs().map_err(|err| Error::LogLayer(err.into()))?;
        }

        self.logger_config
            .take()
            .map(layer::log_layer)
            .map_or(Ok(None), |res| res.map(Some))
    }

    fn build_flamegraph_layer(&mut self) -> Result<(Option<layer::FlamegraphLayer>, Option<Flamegrapher>), Error> {
        self.flamegraph_stack_file
            .take()
            .map_or(Ok((None, None)), |stack_file| {
                layer::flamegraph_layer(stack_file).map(|(layer, flamegrapher)| (Some(layer), Some(flamegrapher)))
            })
    }
}

/// Returns a new, default [`SubscriberBuilder`].
pub fn build() -> SubscriberBuilder {
    SubscriberBuilder::default()
}
