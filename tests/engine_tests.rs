use dynarule::{Condition, Outcome, Rule, RuleEngine};
use serde_json;
use std::collections::HashMap;

#[test]
fn test_basic_evaluation_greater_than() {
    let rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "eligible".to_string(),
            value: serde_json::json!(true),
        },
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "eligible");
    assert_eq!(outcomes[0].value, serde_json::json!(true));

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 0); // Condition fails, no outcome
}

#[test]
fn test_basic_evaluation_equals() {
    let rule = Rule {
        condition: Condition::Simple("status = active".to_string()),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("granted"),
        },
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("status".to_string(), serde_json::json!("active"));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "access");
    assert_eq!(outcomes[0].value, serde_json::json!("granted"));

    let mut input = HashMap::new();
    input.insert("status".to_string(), serde_json::json!("inactive"));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 0);
}

#[test]
fn test_missing_input_key() {
    let rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "eligible".to_string(),
            value: serde_json::json!(true),
        },
    };
    let engine = RuleEngine::new(vec![rule]);

    let input = HashMap::new(); // No "age" key
    let result = engine.evaluate(&input);
    assert!(matches!(
        result,
        Err(dynarule::RuleEngineError::EvaluationError(_))
    ));
}

#[test]
fn test_basic_evaluation_less_than() {
    let rule = Rule {
        condition: Condition::Simple("age < 20".to_string()),
        outcome: Outcome {
            key: "youth".to_string(),
            value: serde_json::json!(true),
        },
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "youth");
    assert_eq!(outcomes[0].value, serde_json::json!(true));
}

#[test]
fn test_nested_and_condition() {
    let rule = Rule {
        condition: Condition::And(vec![
            Condition::Simple("age > 18".to_string()),
            Condition::Simple("status = active".to_string()),
        ]),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("granted"),
        },
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    input.insert("status".to_string(), serde_json::json!("active"));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "access");
    assert_eq!(outcomes[0].value, serde_json::json!("granted"));

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15)); // Fails age condition
    input.insert("status".to_string(), serde_json::json!("active"));
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 0);
}

#[test]
fn test_nested_or_condition() {
    let rule = Rule {
        condition: Condition::Or(vec![
            Condition::Simple("age > 18".to_string()),
            Condition::Simple("status = active".to_string()),
        ]),
        outcome: Outcome {
            key: "eligible".to_string(),
            value: serde_json::json!(true),
        },
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15)); // Fails age, but...
    input.insert("status".to_string(), serde_json::json!("active")); // Status passes
    let outcomes = engine.evaluate(&input).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "eligible");
    assert_eq!(outcomes[0].value, serde_json::json!(true));
}
