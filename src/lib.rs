//! A flexible, JSON-based rule engine for evaluating dynamic conditions.
//!
//! This crate allows users to define rules in JSON and evaluate them against input data.
//! It supports basic comparisons, nested conditions, and custom extensibility.

mod config;
mod engine;
mod error;
pub mod parser;
mod types;

pub use config::Config;
pub use engine::RuleEngine;
pub use error::RuleEngineError;
pub use types::{Condition, Outcome, Rule};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
