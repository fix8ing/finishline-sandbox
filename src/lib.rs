//! Library core for the `finishline` E2E sandbox.
//!
//! The logic here is deliberately trivial. What matters is that the crate
//! builds and passes its checks in the clean state, so that a staged commit
//! can flip exactly one check (fmt, clippy, or test) into failure.

/// Minimal configuration struct.
///
/// `tests/integration.rs` constructs this struct with a positional field
/// literal on purpose: adding a field here breaks only the test compilation,
/// which is the tier-2 scenario finishline's constrained agent is meant to fix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Human-readable name for the config.
    pub name: String,
    /// Number of times an operation should be retried.
    pub retries: u32,
    /// Whether verbose logging is enabled.
    pub verbose: bool,
}

impl Config {
    /// Build a new `Config` from a name and retry count.
    pub fn new(name: impl Into<String>, retries: u32) -> Self {
        Self {
            name: name.into(),
            retries,
            verbose: false,
        }
    }

    /// Total number of attempts, i.e. the first try plus every retry.
    pub fn total_attempts(&self) -> u32 {
        self.retries + 1
    }

    /// Whether any retries are configured at all.
    pub fn is_retrying(&self) -> bool {
        self.retries > 0
    }

    /// A short label combining the name and the attempt budget.
    pub fn describe(&self) -> String {
        format!("{} ({} attempts)", self.name, self.total_attempts())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_attempts_counts_first_try() {
        let config = Config::new("svc", 3);
        assert_eq!(config.total_attempts(), 4);
    }

    #[test]
    fn is_retrying_reflects_retry_count() {
        assert!(!Config::new("svc", 0).is_retrying());
        assert!(Config::new("svc", 1).is_retrying());
    }

    #[test]
    fn describe_includes_name_and_attempts() {
        let config = Config::new("svc", 1);
        assert_eq!(config.describe(), "svc (2 attempts)");
    }
}
