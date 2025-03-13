use dynarule::RuleEngine;
use std::collections::HashMap;

fn main() {
    let json = r#"
    [
        {"condition": {"expr": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
    ]
    "#;
    let rules = dynarule::parser::parse_rules(json).unwrap();
    let engine = RuleEngine::new(rules);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    println!("Outcomes: {:?}", outcomes);
}
