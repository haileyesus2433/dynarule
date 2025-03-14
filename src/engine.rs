use crate::error::RuleEngineError;
use crate::types::{Condition, Outcome, Rule};
use std::cell::RefCell;
use std::collections::HashMap;
use tera::{Context, Tera};

type CustomFunction =
    Box<dyn Fn(&serde_json::Value) -> Result<serde_json::Value, RuleEngineError> + Send + Sync>;

pub struct RuleEngine {
    pub rules: Vec<Rule>,
    custom_functions: HashMap<String, CustomFunction>,
    stop_on_first_match: bool,
    tera: RefCell<Tera>, // Wrap in RefCell for interior mutability
}

impl RuleEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        RuleEngine {
            rules,
            custom_functions: HashMap::new(),
            stop_on_first_match: false,
            tera: RefCell::new(tera),
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

    pub fn with_stop_on_first_match(mut self, value: bool) -> Self {
        self.stop_on_first_match = value;
        self
    }

    pub fn update_rules(&mut self, rules: Vec<Rule>) {
        self.rules = rules;
    }

    pub fn evaluate(
        &self,
        input: &HashMap<String, serde_json::Value>,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<Outcome>, RuleEngineError> {
        let mut outcomes = Vec::new();
        let mut sorted_rules = self.rules.clone();
        sorted_rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        let tera_context = Self::create_tera_context(input, context)?;

        for rule in &sorted_rules {
            if Self::evaluate_condition(&rule.condition, input, context, &self.custom_functions)? {
                let processed_outcome = Self::process_outcome(
                    &rule.outcome,
                    &mut self.tera.borrow_mut(),
                    &tera_context,
                )?;
                outcomes.push(processed_outcome);
                if self.stop_on_first_match {
                    break;
                }
            }
        }
        Ok(outcomes)
    }

    fn create_tera_context(
        input: &HashMap<String, serde_json::Value>,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<Context, RuleEngineError> {
        let mut tera_context = Context::new();
        for (key, value) in input {
            tera_context.insert(key, value);
        }
        for (key, value) in context {
            tera_context.insert(key, value);
        }
        Ok(tera_context)
    }

    fn process_outcome(
        outcome: &Outcome,
        tera: &mut Tera,
        context: &Context,
    ) -> Result<Outcome, RuleEngineError> {
        if let serde_json::Value::String(template) = &outcome.value {
            let rendered = tera.render_str(template, context).map_err(|e| {
                RuleEngineError::EvaluationError(format!("Template rendering failed: {}", e))
            })?;
            Ok(Outcome {
                key: outcome.key.clone(),
                value: serde_json::Value::String(rendered),
            })
        } else {
            Ok(outcome.clone())
        }
    }

    fn evaluate_condition(
        condition: &Condition,
        input: &HashMap<String, serde_json::Value>,
        context: &HashMap<String, serde_json::Value>,
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
                    let arg_value = input
                        .get(arg_key)
                        .or_else(|| context.get(arg_key))
                        .ok_or_else(|| {
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
                        .or_else(|| context.get(left))
                        .ok_or_else(|| {
                            RuleEngineError::EvaluationError(format!("Key '{}' not found", left))
                        })?
                        .clone()
                };

                match operator {
                    ">" | "<" | ">=" | "<=" => {
                        // Try to convert input_value to f64, handling both numbers and strings
                        let input_num = match input_value {
                            serde_json::Value::Number(n) => n.as_f64().ok_or_else(|| {
                                RuleEngineError::EvaluationError("Invalid number".to_string())
                            })?,
                            serde_json::Value::String(s) => s.parse::<f64>().map_err(|e| {
                                RuleEngineError::EvaluationError(format!(
                                    "Cannot parse '{}' as number: {}",
                                    s, e
                                ))
                            })?,
                            _ => {
                                return Err(RuleEngineError::EvaluationError(
                                    "Value must be a number or numeric string".to_string(),
                                ));
                            }
                        };
                        let cond_num = right.parse::<f64>().map_err(|e| {
                            RuleEngineError::EvaluationError(format!(
                                "Right-hand side '{}' must be a number: {}",
                                right, e
                            ))
                        })?;
                        match operator {
                            ">" => Ok(input_num > cond_num),
                            "<" => Ok(input_num < cond_num),
                            ">=" => Ok(input_num >= cond_num),
                            "<=" => Ok(input_num <= cond_num),
                            _ => unreachable!(),
                        }
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
                    if !Self::evaluate_condition(cond, input, context, custom_functions)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Condition::Or(conditions) => {
                for cond in conditions {
                    if Self::evaluate_condition(cond, input, context, custom_functions)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }
}
