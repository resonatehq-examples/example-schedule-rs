//! Run this binary once to create the schedule on the Resonate server.
//! The server will then trigger `generate_report` according to the cron
//! expression — separately from this process. Start the worker (in
//! `worker.rs`) to actually execute each tick.

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

    // Schedule generate_report to run every minute.
    // Change the cron expression to "0 9 * * *" for daily at 9am, etc.
    let result = resonate
        .schedule(
            "daily_report",  // schedule ID
            "* * * * *",     // cron: every minute
            "generate_report", // function name (matches #[resonate::function])
            123_u64,         // user_id argument
        )
        .await;

    match result {
        Ok(_) => println!("Schedule created. Start the worker to process executions."),
        Err(Error::ServerError { code: 40901, .. }) => {
            println!("Schedule already exists. Start the worker to process executions.");
        }
        Err(e) => {
            eprintln!("Failed to create schedule: {e}");
            std::process::exit(1);
        }
    }

    resonate.stop().await.ok();
}
