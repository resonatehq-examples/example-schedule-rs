<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="./assets/banner-light.png">
  <img alt="Schedule banner" src="./assets/banner-light.png">
</picture>

<p align="center">
  <a href="https://resonatehq.github.io/examples-ci/">
    <img src="https://img.shields.io/endpoint?url=https://resonatehq.github.io/examples-ci/status/example-schedule-rs.json" alt="examples-ci status">
  </a>
</p>

# Schedule

**Resonate Rust SDK**

Schedule a Rust function to run periodically using Resonate's high-level `schedule()` API.

Instructions on [How to run this example](#how-to-run-the-example) are below. The full pattern is documented at [docs.resonatehq.io/get-started/examples/schedule](https://docs.resonatehq.io/get-started/examples/schedule).

## What problem does this solve?

Running a function on a cron schedule sounds simple — but in practice, what happens when the worker crashes mid-execution? Traditional cron jobs offer no crash recovery: the job just doesn't run (or runs again from scratch on the next tick). Resonate makes scheduled executions durable. Each cron tick fires a new durable promise. If the worker crashes while processing it, Resonate retries automatically. No lost ticks, no manual recovery logic.

## Overview

This example shows how to use Resonate's `schedule()` method to register a function as a periodic job using a cron expression. The Resonate server triggers the function automatically, and a worker processes each execution durably (`src/bin/schedule.rs`):

<!-- sotto self:src/bin/schedule.rs#register+schedule -->

```rust
resonate.register(generate_report).unwrap();

// Schedule generate_report to run every minute.
// Change the cron expression to "0 9 * * *" for daily at 9am, etc.
let result = resonate
    .schedule(
        "daily_report",    // schedule ID
        "* * * * *",       // cron: every minute
        "generate_report", // function name (matches #[resonate::function])
        123_u64,           // user_id argument
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
```

## How It Works

| File | Role |
|------|------|
| `src/bin/schedule.rs` | Creates the cron schedule on the Resonate server (run once) |
| `src/bin/worker.rs` | Processes each tick (run continuously) |
| `src/lib.rs` | The function that runs on each scheduled tick |

Both binaries call `register` — that is how each process learns the name-to-function mapping for its own side of the job. Registering in `schedule.rs` is what lets it schedule by the name `generate_report`; registering in `worker.rs` is what lets it execute the ticks. Neither one registers on the other's behalf.

Creating the schedule and processing it are deliberately separate: the schedule lives on the server, so `schedule.rs` exits as soon as it has been created, and the workers can come and go independently.

## Cron Reference

| Expression | Meaning |
|------------|---------|
| `* * * * *` | Every minute |
| `0 9 * * *` | Daily at 9am |
| `0 9 * * 1-5` | Weekdays at 9am |
| `*/30 * * * *` | Every 30 minutes |

## How to run the example

This example uses [Cargo](https://www.rust-lang.org/tools/install) as the build tool. After cloning, change directory into the project root.

This example application requires that a Resonate Server is running locally.

```shell
brew install resonatehq/tap/resonate
resonate dev
```

You will need 2 terminals to run this example, one to create the schedule and one for the Worker.

In _Terminal 1_, create the cron schedule on the Resonate server (run once):

```shell
cargo run --bin schedule
```

In _Terminal 2_, start the Worker to process each tick:

```shell
cargo run --bin worker
```

Every minute, you'll see output like:

```
[2026-04-25T09:00:00+00:00] Report for user 123
[2026-04-25T09:01:00+00:00] Report for user 123
```

## Learn more

- [Resonate Documentation](https://docs.resonatehq.io)
- [Schedule Pattern](https://docs.resonatehq.io/get-started/examples/schedule)
- [Schedules API](https://docs.resonatehq.io/learn/schedules)
- [Rust SDK Guide](https://docs.resonatehq.io/develop/rust)
