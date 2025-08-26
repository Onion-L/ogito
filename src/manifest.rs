use color_eyre::{eyre::Ok, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub templates: BTreeMap<String, Template>,
}

impl Manifest {
    pub fn add_template(&mut self, name: String, template: Template) {
        self.templates.insert(name, template);
    }

    pub fn remove_template(&mut self, name: &String) -> Option<Template> {
        // First try to remove by exact name
        if let Some(template) = self.templates.remove(name) {
            return Some(template);
        }

        // If not found by exact name, try to find by alias
        if let Some((actual_name, _)) = self
            .templates
            .iter()
            .find(|(_, template)| template.alias.as_ref() == Some(name))
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            return self.templates.remove(&actual_name);
        }

        None
    }

    pub fn find(&self, name: &String) -> Option<&String> {
        self.templates
            .get_key_value(name)
            .map(|(k, _)| k)
            .or_else(|| {
                self.templates
                    .iter()
                    .find(|(_, template)| template.alias.as_ref() == Some(name))
                    .map(|(name, _)| name)
            })
    }

    pub fn clear(&mut self) {
        self.templates.clear();
    }
}

pub struct ManifestFile {
    pub path: PathBuf,
    pub content: Manifest,
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

    pub fn remove_template(&mut self, name: &String) -> Option<Template> {
        self.content.remove_template(name)
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

    #[test]
    fn test_remove_template_by_name() {
        let mut config = Manifest::default();
        let template = Template {
            description: Some("A test template".to_string()),
            alias: Some("test-alias".to_string()),
            url: "https://example.com/test.git".to_string(),
        };
        config.add_template("my-template".to_string(), template.clone());
        assert_eq!(config.templates.len(), 1);

        let removed = config.remove_template(&"my-template".to_string());
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), template);
        assert_eq!(config.templates.len(), 0);
    }

    #[test]
    fn test_remove_template_by_alias() {
        let mut config = Manifest::default();
        let template = Template {
            description: Some("A test template".to_string()),
            alias: Some("test-alias".to_string()),
            url: "https://example.com/test.git".to_string(),
        };
        config.add_template("my-template".to_string(), template.clone());
        assert_eq!(config.templates.len(), 1);

        let removed = config.remove_template(&"test-alias".to_string());
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), template);
        assert_eq!(config.templates.len(), 0);
    }

    #[test]
    fn test_remove_template_not_found() {
        let mut config = Manifest::default();
        let template = Template {
            description: Some("A test template".to_string()),
            alias: Some("test-alias".to_string()),
            url: "https://example.com/test.git".to_string(),
        };
        config.add_template("my-template".to_string(), template);
        assert_eq!(config.templates.len(), 1);

        let removed = config.remove_template(&"non-existent".to_string());
        assert!(removed.is_none());
        assert_eq!(config.templates.len(), 1);
    }

    #[test]
    fn test_remove_template_prefers_name_over_alias() {
        let mut config = Manifest::default();
        let template1 = Template {
            description: Some("Template 1".to_string()),
            alias: Some("shared-alias".to_string()),
            url: "https://example.com/template1.git".to_string(),
        };
        let template2 = Template {
            description: Some("Template 2".to_string()),
            alias: None,
            url: "https://example.com/template2.git".to_string(),
        };

        config.add_template("shared-alias".to_string(), template1.clone());
        config.add_template("template2".to_string(), template2.clone());
        assert_eq!(config.templates.len(), 2);

        // Should remove by exact name "shared-alias" rather than by alias
        let removed = config.remove_template(&"shared-alias".to_string());
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), template1);
        assert_eq!(config.templates.len(), 1);
    }
}
