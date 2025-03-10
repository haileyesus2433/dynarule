use crate::error::RuleEngineError;
use crate::types::{Condition, Outcome, Rule};
use std::collections::HashMap;

type CustomFunction =
    Box<dyn Fn(&serde_json::Value) -> Result<serde_json::Value, RuleEngineError> + Send + Sync>;

pub struct RuleEngine {
    rules: Vec<Rule>,
    custom_functions: HashMap<String, CustomFunction>,
}

impl RuleEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        RuleEngine {
            rules,
            custom_functions: HashMap::new(),
        }
    }

    pub fn with_function<F>(mut self, name: &str, func: F) -> Self
    where
        F: Fn(&serde_json::Value) -> Result<serde_json::Value, RuleEngineError>
            + 'static
            + Send
            + Sync,
    {
        self.custom_functions
            .insert(name.to_string(), Box::new(func));
        self
    }

    pub fn evaluate(
        &self,
        input: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<Outcome>, RuleEngineError> {
        let mut outcomes = Vec::new();
        for rule in &self.rules {
            if Self::evaluate_condition(&rule.condition, input, &self.custom_functions)? {
                outcomes.push(rule.outcome.clone());
            }
        }
        Ok(outcomes)
    }

    fn evaluate_condition(
        condition: &Condition,
        input: &HashMap<String, serde_json::Value>,
        custom_functions: &HashMap<String, CustomFunction>,
    ) -> Result<bool, RuleEngineError> {
        match condition {
            Condition::Simple(expr) => {
                let parts: Vec<&str> = expr.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(RuleEngineError::EvaluationError(
                        "Simple condition must have 3 parts: key operator value".to_string(),
                    ));
                }

                let left = parts[0];
                let operator = parts[1];
                let right = parts[2];

                let input_value = if left.contains('(') && left.ends_with(')') {
                    let func_name = left[..left.find('(').unwrap()].to_string();
                    let arg_key = &left[left.find('(').unwrap() + 1..left.len() - 1];
                    let arg_value = input.get(arg_key).ok_or_else(|| {
                        RuleEngineError::EvaluationError(format!("Key '{}' not found", arg_key))
                    })?;
                    if let Some(func) = custom_functions.get(&func_name) {
                        func(arg_value)?
                    } else {
                        return Err(RuleEngineError::EvaluationError(format!(
                            "Unknown function '{}'",
                            func_name
                        )));
                    }
                } else {
                    input
                        .get(left)
                        .ok_or_else(|| {
                            RuleEngineError::EvaluationError(format!("Key '{}' not found", left))
                        })?
                        .clone()
                };

                match operator {
                    ">" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = right
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num > cond_num)
                    }
                    "<" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = right
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num < cond_num)
                    }
                    ">=" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = right
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num >= cond_num)
                    }
                    "<=" => {
                        let input_num = input_value.as_f64().ok_or_else(|| {
                            RuleEngineError::EvaluationError("Value must be a number".to_string())
                        })?;
                        let cond_num = right
                            .parse::<f64>()
                            .map_err(|e| RuleEngineError::EvaluationError(e.to_string()))?;
                        Ok(input_num <= cond_num)
                    }
                    "=" => Ok(input_value == serde_json::Value::String(right.to_string())),
                    _ => Err(RuleEngineError::EvaluationError(format!(
                        "Unsupported operator '{}'",
                        operator
                    ))),
                }
            }
            Condition::And(conditions) => {
                for cond in conditions {
                    if !Self::evaluate_condition(cond, input, custom_functions)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Condition::Or(conditions) => {
                for cond in conditions {
                    if Self::evaluate_condition(cond, input, custom_functions)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }
}
