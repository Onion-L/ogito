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

    let mut config = TomlConfig::load(&template_path)?;
    let template = Template {
        description: description.cloned(),
        alias: alias.cloned(),
        url: url.clone(),
    };

    config.add_template(name.clone(), template);
    config.save()?;

    Ok(())
}
