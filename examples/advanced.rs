use dynarule::{Condition, Outcome, Rule, RuleEngine};
use serde_json;
use std::collections::HashMap;

fn main() {
    let rule = Rule {
        condition: Condition::And(vec![
            Condition::Simple("age > 18".to_string()),
            Condition::Or(vec![
                Condition::Simple("status = active".to_string()),
                Condition::Simple("length(name) > 3".to_string()),
            ]),
        ]),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("granted"),
        },
    };
    let engine = RuleEngine::new(vec![rule]).with_function("length", |value| {
        let len = value
            .as_str()
            .ok_or_else(|| {
                dynarule::RuleEngineError::EvaluationError("Expected string".to_string())
            })?
            .len();
        Ok(serde_json::Value::Number(serde_json::Number::from(len)))
    });

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    input.insert("status".to_string(), serde_json::json!("inactive"));
    input.insert("name".to_string(), serde_json::json!("Alexander")); // Length > 3
    let outcomes = engine.evaluate(&input).unwrap();
    println!("Outcomes: {:?}", outcomes); // Should print "access": "granted"
}
