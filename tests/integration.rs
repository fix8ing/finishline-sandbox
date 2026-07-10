//! Integration test that constructs `Config` with an explicit struct literal.
//!
//! This is the tier-2 trap: the literal names every field, so adding a field
//! to `Config` (without updating this file) fails ONLY this test's compilation.
//! finishline's constrained agent is expected to repair it.

use sandbox::Config;

#[test]
fn constructs_config_directly() {
    let config = Config {
        name: "integration".to_string(),
        retries: 5,
    };

    assert_eq!(config.name, "integration");
    assert_eq!(config.retries, 5);
    assert_eq!(config.total_attempts(), 6);
    assert!(config.is_retrying());
    assert_eq!(config.describe(), "integration (6 attempts)");
}
