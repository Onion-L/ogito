mod clone;
mod local;

use crate::regex::is_valid_url;
use clap::ArgMatches;
use clone::direct_clone;
use color_eyre::{eyre::eyre, Result};
use local::local_template;

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let source = matches
        .get_one::<String>("source")
        .ok_or_else(|| eyre!("Source is required"))?;

    if is_valid_url(source)? {
        direct_clone(matches, source).await?
    } else {
        local_template(matches, source).await?
    }

    Ok(())
}
