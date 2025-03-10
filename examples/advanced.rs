use dynarule::{Condition, Config, Outcome, Rule, RuleEngine};
use serde_json;
use std::collections::HashMap;
use std::fs;

fn main() {
    let initial_rules = vec![Rule {
        condition: Condition::Simple("age > 10".to_string()),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("Welcome, {{name}}!"),
        },
        priority: 1,
    }];
    let mut engine = RuleEngine::new(initial_rules).with_function("length", |value| {
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
    input.insert("name".to_string(), serde_json::json!("Haile"));
    let outcomes = engine.evaluate(&input).unwrap();
    println!("Initial outcomes: {:?}", outcomes); // "Welcome, Haile!"

    let new_rules_json = r#"
    [
        {"condition": {"type": "And", "value": [
            {"type": "Simple", "value": "age > 18"},
            {"type": "Simple", "value": "length(name) > 3"}
        ]}, "outcome": {"key": "access", "value": "Premium user: {{name}}"}, "priority": 10}
    ]
    "#;
    fs::write("rules.json", new_rules_json).unwrap();
    Config::reload_from_file("rules.json", &mut engine).unwrap();

    input.insert("name".to_string(), serde_json::json!("Haileyesus"));
    let outcomes = engine.evaluate(&input).unwrap();
    println!("Updated outcomes: {:?}", outcomes); // "Premium user: Haileyesus"
}
