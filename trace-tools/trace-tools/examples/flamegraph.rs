// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{path::PathBuf, time::Duration};

use tokio::{task, time::timeout};
use trace_tools::observe;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");

    let stack_filename = examples_dir.clone().join("trace");

    println!(
        "Creating flamegraph layer, recording to {}.folded",
        stack_filename.display()
    );

    let flamegrapher = trace_tools::subscriber::build()
        .with_flamegraph_layer(stack_filename)
        .init()?
        .unwrap();

    println!("Running tasks for 5 seconds...");
    let task1 = task::spawn(timeout(Duration::from_secs(5), run_tasks(500)));
    let task2 = task::spawn(timeout(Duration::from_secs(5), run_tasks(200)));
    let _ = tokio::join!(task1, task2);

    let graph_filename = examples_dir.join("flamegraph");

    println!("Creating flamegraph at {}.svg", graph_filename.display());

    flamegrapher.with_graph_file(graph_filename)?.write_flamegraph()?;

    Ok(())
}

async fn run_tasks(busy_per_sec: u64) {
    loop {
        let idle_per_sec = 1000 - busy_per_sec;

        tokio::join!(
            full_duration_task(busy_per_sec, idle_per_sec),
            half_duration_task(busy_per_sec, idle_per_sec),
        );
    }
}

#[observe]
async fn full_duration_task(busy: u64, idle: u64) {
    let half_busy = busy / 2;
    let idle_per_sec = 1000 - half_busy;
    half_duration_task(half_busy, idle_per_sec).await;

    std::thread::sleep(Duration::from_millis(busy));
    tokio::time::sleep(Duration::from_millis(idle)).await;
}

#[observe]
async fn half_duration_task(busy: u64, idle: u64) {
    let half_busy = busy / 2;
    let idle_per_sec = 1000 - half_busy;
    quarter_duration_task(half_busy, idle_per_sec).await;

    std::thread::sleep(Duration::from_millis(busy));
    tokio::time::sleep(Duration::from_millis(idle)).await;
}

#[observe]
async fn quarter_duration_task(busy: u64, idle: u64) {
    std::thread::sleep(Duration::from_millis(busy));
    tokio::time::sleep(Duration::from_millis(idle)).await;
}
