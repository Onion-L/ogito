use crate::file::cache::get_cache_root;
use crate::manifest::Manifest;
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use std::fs;

pub async fn local_template(_matches: &ArgMatches, template_name: &String) -> Result<()> {
    let config_path = get_cache_root().join("template.toml");

    if !config_path.exists() {
        return Err(eyre!(
            "No templates configured. Use 'ogito add' to add one."
        ));
    }

    let toml_content = fs::read_to_string(config_path)?;
    let toml_config: Manifest = toml::from_str(&toml_content)?;
    println!("{:?}", toml_config);

    todo!("Implement local template {}", template_name);
}
