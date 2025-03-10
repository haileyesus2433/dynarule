use serde::{Deserialize, Serialize};

/// A single rule with a condition and outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub condition: Condition,
    pub outcome: Outcome,
}

/// A condition to evaluate (e.g., "age > 18").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub expr: String, // For now, a simple string like "key operator value"
}

/// The result of a rule evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub key: String,              // The key to set (e.g., "eligible")
    pub value: serde_json::Value, // Flexible value (string, number, etc.)
}
