use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RuleEngineError {
    ParseError(String),
    EvaluationError(String),
    ConfigError(String),
}

impl fmt::Display for RuleEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleEngineError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RuleEngineError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            RuleEngineError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl Error for RuleEngineError {}
