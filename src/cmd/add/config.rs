use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempConfig {
    templates: HashMap<String, Template>,
}

impl TempConfig {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, name: String, template: Template) {
        self.templates.insert(name, template);
    }
}

pub struct TomlConfig {
    toml_path: PathBuf,
}

impl TomlConfig {
    pub fn new(toml_path: PathBuf) -> Self {
        Self {
            toml_path: toml_path,
        }
    }

    pub fn read_file(&self) -> Result<TempConfig> {
        let file_content = fs::read_to_string(&self.toml_path)?;
        let toml_content: TempConfig = toml::from_str(&file_content)?;
        Ok(toml_content)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    pub description: Option<String>,
    pub alias: Option<String>,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_template() {
        let mut config = TempConfig::new();
        let template = Template {
            description: Some("A test template".to_string()),
            alias: Some("test".to_string()),
            url: "https://example.com/test.git".to_string(),
        };
        config.add_template("my-template".to_string(), template.clone());
        assert_eq!(config.templates.len(), 1);
        let retrieved_template = config.templates.get("my-template").unwrap();
        assert_eq!(*retrieved_template, template);
    }
}
