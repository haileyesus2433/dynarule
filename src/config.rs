use crate::error::RuleEngineError;
use crate::parser;
use std::fs;

pub struct Config;

impl Config {
    pub fn load_from_file(path: &str) -> Result<Vec<crate::types::Rule>, RuleEngineError> {
        let content = fs::read_to_string(path)
            .map_err(|e| RuleEngineError::ConfigError(format!("Failed to read file: {}", e)))?;
        parser::parse_rules(&content)
    }

    pub fn reload_from_file(
        path: &str,
        engine: &mut crate::engine::RuleEngine,
    ) -> Result<(), RuleEngineError> {
        let rules = Self::load_from_file(path)?;
        engine.rules = rules;
        Ok(())
    }
}
