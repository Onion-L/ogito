use clap::ArgMatches;
use color_eyre::{
    eyre::{eyre, OptionExt},
    Result,
};
use std::{fs, io::Write};

use crate::file::cache::get_cache_root;

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("url")
        .ok_or_eyre("URL is required")?;
    let name = matches.get_one::<String>("name");
    let description = matches.get_one::<String>("description");
    let alias = matches.get_one::<String>("alias");
    let force = matches.get_flag("force");
    let update = matches.get_flag("update");

    let root_path = get_cache_root();
    let template_path = root_path.join("template.toml");
    let mut temp_config = fs::File::create(template_path)?;
    temp_config.write_all(url.as_bytes())?;

    Ok(())
}
