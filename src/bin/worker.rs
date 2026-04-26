//! Run this binary to process scheduled executions.
//! The worker registers the function and waits for tasks dispatched by
//! the Resonate server on each cron tick.

use example_schedule_rs::generate_report;
use resonate::prelude::*;

#[tokio::main]
async fn main() {
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("workers".into()),
        ..Default::default()
    });

    resonate.register(generate_report).unwrap();

    println!("Worker started. Waiting for scheduled executions...");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");

    resonate.stop().await.ok();
}
