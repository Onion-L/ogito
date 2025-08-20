mod config;

use crate::{
    file::cache::get_cache_root,
    mode::Mode,
    regex::{extract_path, is_valid_url},
};
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use config::{Template, TomlConfig};
use console::style;
use std::fs;

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("url")
        .ok_or_else(|| eyre!("URL is required"))?;

    if !is_valid_url(url)? {
        return Err(eyre!("Invalid URL: {}", url));
    }

    let name = match matches.get_one::<String>("name") {
        Some(name) => name.clone(),
        None => generate_default_name(url)?,
    };

    let template = build_template(matches, url)?;
    let templates_dir = get_cache_root().join("templates");
    let destination = templates_dir.join(&name);

    if destination.exists() {
        println!(
            "{}",
            style(format!("⚠️ {} already exists. Skipping clone.", &name)).yellow()
        );
    } else {
        fs::create_dir_all(&destination)?;
        let dest_string = destination
            .to_str()
            .ok_or_else(|| eyre!("Failed to convert destination path to string"))?
            .to_string();

        let clone_config =
            crate::fetch::config::Config::from(&dest_string, Mode::Git, false, true, None);

        crate::clone::clone(&template.url, &clone_config).await?;
    }

    let config_path = get_cache_root().join("template.toml");
    let mut config = TomlConfig::load(&config_path)?;
    config.add_template(name.clone(), template.clone());
    config.save()?;

    Ok(())
}

fn build_template(matches: &ArgMatches, url: &String) -> Result<Template> {
    let template = Template {
        description: matches.get_one::<String>("description").cloned(),
        alias: matches.get_one::<String>("alias").cloned(),
        url: url.clone(),
    };

    Ok(template)
}

fn generate_default_name(url: &String) -> Result<String> {
    let (owner, repo) =
        extract_path(url).ok_or_else(|| eyre!("Could not extract path from URL"))?;
    Ok(format!("{}/{}", owner, repo))
}
