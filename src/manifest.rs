use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    templates: BTreeMap<String, Template>,
}

impl Manifest {
    pub fn add_template(&mut self, name: String, template: Template) {
        self.templates.insert(name, template);
    }
}

pub struct ManifestFile {
    path: PathBuf,
    content: Manifest,
}

impl ManifestFile {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            let content = Manifest::default();
            let toml_content = toml::to_string_pretty(&content)?;
            fs::write(path, toml_content)?;
        }

        let file_content = fs::read_to_string(path)?;
        let content: Manifest = toml::from_str(&file_content)?;
        Ok(Self {
            path: path.to_path_buf(),
            content,
        })
    }

    pub fn add_template(&mut self, name: String, template: Template) {
        self.content.add_template(name, template);
    }

    pub fn save(&self) -> Result<()> {
        let toml_content = toml::to_string_pretty(&self.content)?;
        fs::write(&self.path, toml_content)?;
        Ok(())
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
        let mut config = Manifest::default();
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
