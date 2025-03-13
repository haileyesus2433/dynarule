use crate::error::RuleEngineError;
use crate::types::Rule;

pub fn parse_rules(json: &str) -> Result<Vec<Rule>, RuleEngineError> {
    serde_json::from_str(json)
        .map_err(|e| RuleEngineError::ParseError(format!("Failed to parse JSON: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_json() {
        let json = r#"
        [
            {"condition": {"type": "Simple", "value": "age > 18"}, "outcome": {"key": "eligible", "value": true}}
        ]
        "#;
        let rules = parse_rules(json).unwrap();
        assert_eq!(rules.len(), 1);
        // assert_eq!(rules[0].condition, "age > 18");
        assert_eq!(rules[0].outcome.key, "eligible");
        assert_eq!(rules[0].outcome.value, serde_json::Value::Bool(true));
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = "invalid json";
        let result = parse_rules(json);
        assert!(matches!(result, Err(RuleEngineError::ParseError(_))));
    }
}
