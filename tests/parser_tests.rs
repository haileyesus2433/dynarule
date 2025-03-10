use dynarule::{Condition, Rule, parser};

#[test]
fn test_parse_valid_json() {
    let json = r#"
    [
        {"condition": {"type": "Simple", "value": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
    ]
    "#;
    let rules = parser::parse_rules(json).unwrap();
    assert_eq!(rules.len(), 1);
    if let dynarule::Condition::Simple(expr) = &rules[0].condition {
        assert_eq!(expr, "age > 18");
    }
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

#[test]
fn test_parse_nested_json() {
    let json = r#"
    [
        {
            "condition": {
                "type": "And",
                "value": [
                    {"type": "Simple", "value": "age > 18"},
                    {"type": "Simple", "value": "status = active"}
                ]
            },
            "outcome": {"key": "access", "value": "granted"}
        }
    ]
    "#;
    let rules = parser::parse_rules(json).unwrap();
    assert_eq!(rules.len(), 1);
    if let Condition::And(conds) = &rules[0].condition {
        assert_eq!(conds.len(), 2);
        if let Condition::Simple(expr) = &conds[0] {
            assert_eq!(expr, "age > 18");
        }
    }
    assert_eq!(rules[0].outcome.key, "access");
}
