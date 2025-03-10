use dynarule::{Rule, parser};

#[test]
fn test_parse_valid_json() {
    let json = r#"
    [
        {"condition": {"expr": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
    ]
    "#;
    let rules = parser::parse_rules(json).unwrap();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].condition.expr, "age > 18");
    assert_eq!(rules[0].outcome.key, "eligible");
    assert_eq!(rules[0].outcome.value, serde_json::json!(true));
}

#[test]
fn test_parse_invalid_json() {
    let json = r#"{"invalid": "json"}"#; // Not an array of rules
    let result = parser::parse_rules(json);
    assert!(matches!(
        result,
        Err(dynarule::RuleEngineError::ParseError(_))
    ));
}
