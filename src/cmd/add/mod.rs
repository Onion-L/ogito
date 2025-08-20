mod config;
use config::{TempConfig, Template};

use crate::{
    file::cache::get_cache_root,
    regex::{extract_host, extract_path, is_valid_url},
};
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use std::{fs, path::PathBuf};

struct TomlConfig {
    toml_path: PathBuf,
}

impl TomlConfig {
    fn new(toml_path: PathBuf) -> Self {
        Self {
            toml_path: toml_path,
        }
    }

    fn read_file(&self) -> Result<TempConfig> {
        let file_content = fs::read_to_string(&self.toml_path)?;
        let toml_content: TempConfig = toml::from_str(&file_content)?;
        Ok(toml_content)
    }
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("url")
        .ok_or_else(|| eyre!("URL is required"))?;

    if !is_valid_url(url)? {
        return Err(eyre!("Invalid URL"));
    }

    let (owner, repo) = extract_path(url).ok_or_else(|| eyre!("Invalid URL"))?;
    let host = extract_host(url).unwrap_or_else(|| "unknown".to_string());
    let default_name = format!("{}:{}/{}", host, owner, repo);
    let name = match matches.get_one::<String>("name") {
        Some(name) => name.clone(),
        None => default_name,
    };

    let description = matches.get_one::<String>("description");
    let alias = matches.get_one::<String>("alias");
    let _force = matches.get_flag("force");
    let _update = matches.get_flag("update");

    let root_path = get_cache_root();
    let template_path = root_path.join("template.toml");
    if !template_path.exists() {
        fs::write(&template_path, "")?;
    }

    let toml_config = TomlConfig::new(template_path.clone());
    let toml_content = toml_config.read_file()?;
    dbg!(toml_content);

    let template = Template {
        description: description.cloned(),
        alias: alias.cloned(),
        url: url.clone(),
    };

    let mut temp_config = TempConfig::new();
    temp_config.add_template(name, template);

    let toml_content = toml::to_string_pretty(&temp_config)?;

    fs::write(&template_path, toml_content)?;

    Ok(())
}
