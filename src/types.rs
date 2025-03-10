use serde::{Deserialize, Serialize};

/// A single rule with a condition and outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub condition: Condition,
    pub outcome: Outcome,
}

/// A condition to evaluate, either a simple expression or a nested combination.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Condition {
    Simple(String),      // e.g., "age > 18"
    And(Vec<Condition>), // e.g., ["age > 18", "status = active"]
    Or(Vec<Condition>),  // e.g., ["age > 18", "status = inactive"]
}

/// The result of a rule evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub key: String,
    pub value: serde_json::Value,
}
