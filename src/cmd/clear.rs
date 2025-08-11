use clap::ArgMatches;
use color_eyre::Result;
use dialoguer::Confirm;

use crate::file::{file::clear_directory};

pub async fn run(_matches: &ArgMatches) -> Result<()> {
    // TODO: implement clearing cached templates
    let force = _matches.get_flag("force");
    let cache_path = dirs::cache_dir()
        .expect("Failed to get cache directory")
        .join("ogito")
        .join("cache");


    if !force {
        let confirm = Confirm::new().with_prompt("Are you sure you want to clear the cache?").default(false).interact()?;
        if !confirm {
            return Ok(());
        }
    }
    clear_directory(&cache_path)?;
    println!("📦 cache cleared");
    Ok(())
}
