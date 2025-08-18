mod config;
use config::{TempConfig, Template};

use crate::{
    file::cache::get_cache_root,
    regex::{extract_path, is_valid_url},
};
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Write};

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("url")
        .ok_or_else(|| eyre!("URL is required"))?;

    if !is_valid_url(url)? {
        return Err(eyre!("Invalid URL"));
    }

    let (owner, repo) = extract_path(url).ok_or_else(|| eyre!("Invalid URL"))?;
    let name = match matches.get_one::<String>("name") {
        Some(name) => name.clone(),
        None => format!("{}-{}", owner, repo),
    };

    let description = matches.get_one::<String>("description");
    let alias = matches.get_one::<String>("alias");
    let force = matches.get_flag("force");
    let update = matches.get_flag("update");

    let template = Template {
        description: description.cloned(),
        alias: alias.cloned(),
        url: url.clone(),
    };

    let mut temp_config = TempConfig::new();
    temp_config.add_template(name, template);

    let root_path = get_cache_root();
    let temps_path = root_path.join("templates");

    let template_path = root_path.join("template.toml");
    let mut temp_config = fs::File::create(template_path)?;
    temp_config.write_all(url.as_bytes())?;

    Ok(())
}
