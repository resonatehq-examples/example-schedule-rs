// `Result` in the function signature below is `resonate::prelude::Result`,
// resolved by the `#[resonate::function]` macro expansion.
#[allow(unused_imports)]
use resonate::prelude::*;

/// The function that runs on each scheduled tick.
///
/// Resonate fires a new durable promise on every cron tick and dispatches
/// it to a worker. The worker executes `generate_report` and records the
/// result. If the worker crashes mid-execution, Resonate retries — the
/// tick is not lost.
#[resonate::function]
pub async fn generate_report(user_id: u64) -> Result<String> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let report = format!("[{timestamp}] Report for user {user_id}");
    println!("{report}");
    Ok(report)
}
