use dynarule::{Config, RuleEngine};
use std::collections::HashMap;

#[test]
fn test_integration_from_file() {
    // Simulate a file by using a string
    let json = r#"
    [
        {"condition": {"type": "Simple", "value": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
    ]
    "#;
    let rules = dynarule::parser::parse_rules(json).unwrap(); // Use parser directly for test
    let engine = RuleEngine::new(rules);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "eligible");
    assert_eq!(outcomes[0].value, serde_json::json!(true));
}

#[test]
fn test_dynamic_rule_update() {
    let initial_json = r#"
    [
        {"condition": {"type": "Simple", "value": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
    ]
    "#;
    let rules = dynarule::parser::parse_rules(initial_json).unwrap();
    let mut engine = RuleEngine::new(rules);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);

    let new_json = r#"
    [
        {"condition": {"type": "Simple", "value": "age > 30"}, "outcome": {"key": "senior", "value": true}}
    ]
    "#;
    let new_rules = dynarule::parser::parse_rules(new_json).unwrap();
    engine.update_rules(new_rules);
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 0); // Age 25 no longer matches
}
