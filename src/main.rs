//! Thin binary entry point for the `finishline` E2E sandbox.
//!
//! All real logic lives in the library crate so that `tests/integration.rs`
//! can reference [`Config`] directly. Keep this file constructing `Config`
//! the same way the integration test does — the tier-2 scenario adds a struct
//! field here and to the library, but deliberately leaves the test untouched.

use sandbox::Config;

fn main() {
    let config = Config::new("default", 2);
    println!("{}", config.describe());
    if config.is_retrying() {
        println!("retries enabled: {}", config.retries);
    }
}
