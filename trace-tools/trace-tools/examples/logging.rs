// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use fern_logger::{LoggerConfig, LoggerOutputConfigBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = LoggerOutputConfigBuilder::new()
        .name("stdout")
        .level_filter(log::LevelFilter::Info)
        .target_filters(&["logging::filtered"])
        .target_exclusions(&["logging::filtered::excluded"]);

    let log_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/warn.log");

    let warn = LoggerOutputConfigBuilder::new()
        .name(log_file.to_str().unwrap())
        .level_filter(log::LevelFilter::Warn);

    let config = LoggerConfig::build().with_output(stdout).with_output(warn).finish();

    let _ = trace_tools::subscriber::build().with_log_layer(config).init().unwrap();

    log();
    filtered::log();
    filtered::excluded::log();

    Ok(())
}

pub fn log() {
    log::info!("This should not log, it is not captured by the filter.");
}

mod filtered {
    pub fn log() {
        log::trace!("This should not log, it is not at INFO level.");
        log::debug!("This should not log, it is not at INFO level.");
        log::info!("This should log at the INFO level.");
        log::warn!("This should log at the WARN level.");
        log::error!("This should log at the ERROR level.");
    }

    pub mod excluded {
        pub fn log() {
            log::warn!("This should only log to \"warn.log\", it has been excluded from stdout.");
            log::error!("This should only log to \"warn.log\", it has been excluded from stdout.");
        }
    }
}
