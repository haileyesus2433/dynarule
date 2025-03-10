use crate::error::RuleEngineError;
use crate::types::{Condition, Outcome, Rule};
use std::collections::HashMap;

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        RuleEngine { rules }
    }

    pub fn evaluate(
        &self,
        input: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<Outcome>, RuleEngineError> {
        let mut outcomes = Vec::new();
        for rule in &self.rules {
            if Self::evaluate_condition(&rule.condition, input)? {
                outcomes.push(rule.outcome.clone());
            }
        }
        Ok(outcomes)
    }

    fn evaluate_condition(
        condition: &Condition,
        input: &HashMap<String, serde_json::Value>,
    ) -> Result<bool, RuleEngineError> {
        match condition {
            Condition::Simple(expr) => {
                let parts: Vec<&str> = expr.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(RuleEngineError::EvaluationError(
                        "Simple condition must have 3 parts: key operator value".to_string(),
                    ));
                }
                let key = parts[0];
                let operator = parts[1];
                let value = parts[2];

                let input_value = input.get(key).ok_or_else(|| {
                    RuleEngineError::EvaluationError(format!("Key '{}' not found", key))
                })?;

                match operator {
                    ">" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = value
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num > cond_num)
                    }
                    "<" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = value
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num < cond_num)
                    }
                    ">=" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = value
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num >= cond_num)
                    }
                    "<=" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = value
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num <= cond_num)
                    }
                    "=" => Ok(input_value == &serde_json::Value::String(value.to_string())),
                    _ => Err(RuleEngineError::EvaluationError(format!(
                        "Unsupported operator '{}'",
                        operator
                    ))),
                }
            }
            Condition::And(conditions) => {
                for cond in conditions {
                    if !Self::evaluate_condition(cond, input)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Condition::Or(conditions) => {
                for cond in conditions {
                    if Self::evaluate_condition(cond, input)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }
}
