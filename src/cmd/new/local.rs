use crate::file::{cache::get_cache_root, copy::copy_template};
use crate::manifest::Manifest;
use clap::ArgMatches;
use color_eyre::eyre::Ok;
use color_eyre::{eyre::eyre, Result};
use std::fs;

pub async fn local_template(_matches: &ArgMatches, template_name: &String) -> Result<()> {
    let cache_path = get_cache_root();
    let config_path = cache_path.join("template.toml");
    let template_path = cache_path.join("templates");

    if !config_path.exists() {
        return Err(eyre!(
            "No templates configured. Use 'ogito add' to add one."
        ));
    }

    let toml_content = fs::read_to_string(&config_path)?;
    let toml_config: Manifest = toml::from_str(&toml_content)?;
    let path_name = toml_config
        .find(template_name)
        .ok_or_else(|| eyre!("Template '{}' not found", template_name))?;
    let source = template_path.join(path_name);

    copy_template(source, template_name)?;

    Ok(())
}
