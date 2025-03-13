use dynarule::{Condition, Outcome, Rule, RuleEngine, RuleEngineError};
use std::collections::HashMap;

#[test]
fn test_basic_evaluation_greater_than() {
    let rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "eligible".to_string(),
            value: serde_json::json!(true),
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "eligible");
    assert_eq!(outcomes[0].value, serde_json::json!(true));

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15));
    let outcomes = engine.evaluate(&input, &context).unwrap();
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
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("status".to_string(), serde_json::json!("active"));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "access");
    assert_eq!(outcomes[0].value, serde_json::json!("granted"));

    let mut input = HashMap::new();
    input.insert("status".to_string(), serde_json::json!("inactive"));
    let outcomes = engine.evaluate(&input, &context).unwrap();
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
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let input = HashMap::new(); // No "age" key
    let context = HashMap::new(); // Empty context for this test

    let result = engine.evaluate(&input, &context);
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
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
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
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    input.insert("status".to_string(), serde_json::json!("active"));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "access");
    assert_eq!(outcomes[0].value, serde_json::json!("granted"));

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15)); // Fails age condition
    input.insert("status".to_string(), serde_json::json!("active"));
    let outcomes = engine.evaluate(&input, &context).unwrap();
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
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(15)); // Fails age, but...
    input.insert("status".to_string(), serde_json::json!("active")); // Status passes
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "eligible");
    assert_eq!(outcomes[0].value, serde_json::json!(true));
}

#[test]
fn test_custom_function() {
    let rule = Rule {
        condition: Condition::Simple("length(name) > 2".to_string()),
        outcome: Outcome {
            key: "valid".to_string(),
            value: serde_json::json!(true),
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]).with_function("length", |value| {
        let len = value
            .as_str()
            .ok_or_else(|| RuleEngineError::EvaluationError("Expected string".to_string()))?
            .len();
        Ok(serde_json::Value::Number(serde_json::Number::from(len)))
    });

    let mut input = HashMap::new();
    input.insert("name".to_string(), serde_json::json!("Haile"));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "valid");
    assert_eq!(outcomes[0].value, serde_json::json!(true));

    let mut input = HashMap::new();
    input.insert("name".to_string(), serde_json::json!("Al")); // Too short
    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 0);
}

#[test]
fn test_unknown_function() {
    let rule = Rule {
        condition: Condition::Simple("unknown(name) > 5".to_string()),
        outcome: Outcome {
            key: "valid".to_string(),
            value: serde_json::json!(true),
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("name".to_string(), serde_json::json!("Haile"));
    let context = HashMap::new(); // Empty context for this test

    let result = engine.evaluate(&input, &context);
    assert!(matches!(result, Err(RuleEngineError::EvaluationError(_))));
}

#[test]
fn test_rule_priority() {
    let low_priority_rule = Rule {
        condition: Condition::Simple("age > 10".to_string()),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("low"),
        },
        priority: 1,
    };
    let high_priority_rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("high"),
        },
        priority: 10,
    };
    let engine =
        RuleEngine::new(vec![low_priority_rule, high_priority_rule]).with_stop_on_first_match(true);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1); // Only one outcome due to stop_on_first_match
    assert_eq!(outcomes[0].value, serde_json::json!("high")); // Higher priority wins
}

#[test]
fn test_no_stop_on_first_match() {
    let low_priority_rule = Rule {
        condition: Condition::Simple("age > 10".to_string()),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("low"),
        },
        priority: 1,
    };
    let high_priority_rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("high"),
        },
        priority: 10,
    };
    let engine = RuleEngine::new(vec![high_priority_rule, low_priority_rule]); // Default: no stop

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let context = HashMap::new(); // Empty context for this test

    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 2); // Both rules match
    assert_eq!(outcomes[0].value, serde_json::json!("high")); // Higher priority first
    assert_eq!(outcomes[1].value, serde_json::json!("low"));
}

#[test]
fn test_templated_outcome() {
    let rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "message".to_string(),
            value: serde_json::json!("Hello, {{name}}!"),
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    input.insert("name".to_string(), serde_json::json!("Alex"));
    let context = HashMap::new();
    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "message");
    assert_eq!(outcomes[0].value, serde_json::json!("Hello, Alex!"));
}

#[test]
fn test_missing_template_key() {
    let rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "message".to_string(),
            value: serde_json::json!("Hello, {{unknown}}!"),
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let context = HashMap::new(); // Empty context for this test
    let result = engine.evaluate(&input, &context);
    assert!(matches!(result, Err(RuleEngineError::EvaluationError(_))));
}

#[test]
fn test_contextual_data() {
    let rule = Rule {
        condition: Condition::Simple("time > 12".to_string()),
        outcome: Outcome {
            key: "message".to_string(),
            value: serde_json::json!("Afternoon, {{name}}!"),
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("name".to_string(), serde_json::json!("Haile"));
    let mut context = HashMap::new();
    context.insert("time".to_string(), serde_json::json!(14)); // 2 PM
    let outcomes = engine.evaluate(&input, &context).unwrap();
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].key, "message");
    assert_eq!(outcomes[0].value, serde_json::json!("Afternoon, Haile!"));
}
