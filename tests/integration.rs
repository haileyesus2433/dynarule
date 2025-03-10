use dynarule::{Config, RuleEngine};
use std::collections::HashMap;

#[test]
fn test_integration_from_file() {
    // Simulate a file by using a string
    let json = r#"
    [
        {"condition": {"expr": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
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
