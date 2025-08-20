mod config;

use crate::file::cache::get_cache_root;
use crate::regex::{extract_host, extract_path, is_valid_url};
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use config::{Template, TomlConfig};

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
    let config_path = get_cache_root().join("template.toml");
    let mut config = TomlConfig::load(&config_path)?;
    config.add_template(name.clone(), template);
    config.save()?;

    Ok(())
}

/// Parses command-line arguments to construct the template name and data.
fn build_template(matches: &ArgMatches, url: &String) -> Result<Template> {
    let template = Template {
        description: matches.get_one::<String>("description").cloned(),
        alias: matches.get_one::<String>("alias").cloned(),
        url: url.clone(),
    };

    Ok(template)
}

/// Generates a default template name from a git URL, e.g., "github.com:user/repo".
fn generate_default_name(url: &String) -> Result<String> {
    let (owner, repo) =
        extract_path(url).ok_or_else(|| eyre!("Could not extract path from URL"))?;
    let host = extract_host(url).unwrap_or_else(|| "unknown".to_string());
    Ok(format!("{}:{}/{}", host, owner, repo))
}
