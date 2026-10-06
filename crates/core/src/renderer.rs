use std::collections::HashMap;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::{
    command,
    config::{Config, Template},
    parser::Node,
};

pub async fn render(config: &Config, template: &Template, mut ctx: Value) -> Result<String> {
    let mut result = String::new();
    let mut values = HashMap::<String, String>::new();
    for node in &template.nodes {
        match node {
            Node::Text(text) => escape(text, &mut result),
            Node::Cursor => result.push_str("$0"),
            Node::Variable(name) => {
                if !values.contains_key(name) {
                    ctx["variable"]["name"] = Value::String(name.clone());
                    let value = if let Some(variable) = config.variable(template, name) {
                        command::execute(variable, &config.directory, &ctx)
                            .await
                            .with_context(|| format!("template {}, variable {name}", template.id))?
                    } else {
                        ctx["builtins"][name].as_str().unwrap().to_owned()
                    };
                    values.insert(name.clone(), value);
                }
                escape(&values[name], &mut result);
            }
        }
    }
    Ok(result)
}

fn escape(text: &str, result: &mut String) {
    for character in text.chars() {
        if matches!(character, '\\' | '$' | '}') {
            result.push('\\');
        }
        result.push(character);
    }
}
