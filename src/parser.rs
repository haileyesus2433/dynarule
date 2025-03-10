use crate::error::RuleEngineError;
use crate::types::Rule;
use serde_json;

pub fn parse_rules(json: &str) -> Result<Vec<Rule>, RuleEngineError> {
    serde_json::from_str(json)
        .map_err(|e| RuleEngineError::ParseError(format!("Failed to parse JSON: {}", e)))
}
