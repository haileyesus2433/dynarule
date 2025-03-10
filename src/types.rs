use serde::{Deserialize, Serialize};

/// A single rule with a condition, outcome, and priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub condition: Condition,
    pub outcome: Outcome,
    #[serde(default = "default_priority")]
    pub priority: i32, // Higher number = higher priority
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            condition: Condition::Simple(String::new()),
            outcome: Outcome {
                key: String::new(),
                value: serde_json::Value::Null,
            },
            priority: default_priority(), // Default to 0
        }
    }
}

fn default_priority() -> i32 {
    0 // Default priority if not specified
}

/// A condition to evaluate, either a simple expression or a nested combination.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Condition {
    Simple(String),
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

/// The result of a rule evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub key: String,
    pub value: serde_json::Value,
}
