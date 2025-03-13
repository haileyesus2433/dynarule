# Dynarule 🚀

A flexible, JSON-based rule engine for Rust, designed for dynamic condition evaluation. Supports nested conditions, custom functions, rule priorities, dynamic updates, and templated outcomes using Tera.

## ✨ Features

- ⚡️ Define rules in JSON
- 🔍 Evaluate conditions with operators (`>`, `<`, `>=`, `<=`, `=`)
- 🌳 Nested conditions with `AND`/`OR`
- 🛠 Custom functions (e.g., `length(name)`)
- ⚖️ Rule prioritization and stop-on-first-match
- 🔄 Dynamic rule reloading from files
- 📝 Templated outcomes (e.g., `"Hello, {{name}}!"`)

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dynarule = "0.1.0"
```

## 📚 Usage

### Simple Rule Evaluation

This example shows a basic rule that checks if an age is over 18:

```rust
use dynarule::{RuleEngine, Rule, Condition, Outcome};
use std::collections::HashMap;
use serde_json;

fn main() {
    let rule = Rule {
        condition: Condition::Simple("age > 18".to_string()),
        outcome: Outcome {
            key: "eligible".to_string(),
            value: serde_json::json!(true)
        },
        ..Default::default()
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    let context = HashMap::new();
    let outcomes = engine.evaluate(&input, &context).unwrap();
    println!("Outcomes: {:?}", outcomes);  // Prints: [Outcome { key: "eligible", value: true }]
}
```

### Advanced Example

This example demonstrates nested conditions, templating, and contextual data:

```rust
use dynarule::{RuleEngine, Rule, Condition, Outcome};
use std::collections::HashMap;
use serde_json;

fn main() {
    let rule = Rule {
        condition: Condition::And(vec![
            Condition::Simple("age > 18".to_string()),
            Condition::Simple("time > 12".to_string()),
        ]),
        outcome: Outcome {
            key: "access".to_string(),
            value: serde_json::json!("Hello, {{name}} at {{time}}!")
        },
        priority: 10,
    };
    let engine = RuleEngine::new(vec![rule]);

    let mut input = HashMap::new();
    input.insert("age".to_string(), serde_json::json!(25));
    input.insert("name".to_string(), serde_json::json!("Haile"));
    let mut context = HashMap::new();
    context.insert("time".to_string(), serde_json::json!(14));
    let outcomes = engine.evaluate(&input, &context).unwrap();
    println!("Outcomes: {:?}", outcomes);  // Prints: [Outcome { key: "access", value: "Hello, Haile at 14!" }]
}
```

## 🗺 Roadmap

- [ ] Rule Chaining
- [ ] Type Safety for Inputs and Outputs
- [ ] Performance Optimization
- [ ] Debugging and Logging
- [ ] Extensibility via Plugins
- [ ] Multi-Format Rule Support

## 📄 License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) at your option.
