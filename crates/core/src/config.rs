use std::{collections::HashMap, path::PathBuf};

use anyhow::{Result, bail, ensure};
use chrono::format::{Item, StrftimeItems};
use serde::Deserialize;

use crate::parser::{self, Node};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub formats: Formats,
    #[serde(default)]
    pub variables: HashMap<String, Variable>,
    #[serde(default)]
    pub templates: Vec<Template>,
    #[serde(skip)]
    pub directory: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Formats {
    pub date: String,
    pub time: String,
    pub timezone: String,
}

impl Default for Formats {
    fn default() -> Self {
        Self {
            date: "%Y/%-m/%-d".into(),
            time: "%H:%M".into(),
            timezone: "local".into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub command: Vec<String>,
    pub cwd: Option<PathBuf>,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_timeout() -> u64 {
    1000
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub id: String,
    pub trigger: String,
    #[serde(default)]
    pub description: String,
    pub languages: Vec<String>,
    pub body: String,
    #[serde(default)]
    pub variables: HashMap<String, Variable>,
    #[serde(skip)]
    pub nodes: Vec<Node>,
}

pub fn is_builtin(name: &str) -> bool {
    matches!(name, "date" | "time" | "host")
}

impl Config {
    pub fn parse(source: &str, directory: PathBuf) -> Result<Self> {
        let mut config: Self = toml::from_str(source)?;
        config.directory = directory;
        ensure!(config.version == 1, "unsupported configuration version");
        ensure!(
            matches!(config.formats.timezone.as_str(), "local" | "UTC"),
            "timezone must be local or UTC"
        );
        for format in [&config.formats.date, &config.formats.time] {
            ensure!(
                !StrftimeItems::new(format).any(|item| matches!(item, Item::Error)),
                "invalid date/time format: {format}"
            );
        }
        validate_variables(&config.variables)?;
        for template in &mut config.templates {
            ensure!(
                !template.trigger.is_empty() && !template.trigger.contains(char::is_whitespace),
                "template {}: trigger must be nonempty and contain no whitespace",
                template.id
            );
            validate_variables(&template.variables)?;
            template.nodes = parser::parse(&template.body)?;
            for node in &template.nodes {
                if let Node::Variable(name) = node {
                    ensure!(
                        is_builtin(name)
                            || template.variables.contains_key(name)
                            || config.variables.contains_key(name),
                        "template {}: undefined variable ${name}$",
                        template.id
                    );
                }
            }
        }
        for (index, template) in config.templates.iter().enumerate() {
            for other in &config.templates[..index] {
                if template.id == other.id {
                    bail!("duplicate template id: {}", template.id);
                }
                if template.trigger == other.trigger
                    && template.languages.iter().any(|language| {
                        other
                            .languages
                            .iter()
                            .any(|other| language.eq_ignore_ascii_case(other))
                    })
                {
                    bail!("duplicate trigger: {}", template.trigger);
                }
            }
        }
        Ok(config)
    }

    pub fn variable<'a>(&'a self, template: &'a Template, name: &str) -> Option<&'a Variable> {
        template
            .variables
            .get(name)
            .or_else(|| self.variables.get(name))
    }
}

fn validate_variables(variables: &HashMap<String, Variable>) -> Result<()> {
    for (name, variable) in variables {
        ensure!(name != "END", "END is reserved for the final cursor");
        ensure!(
            variable
                .command
                .first()
                .is_some_and(|program| !program.is_empty()),
            "variable {name}: command must contain a program"
        );
    }
    Ok(())
}
