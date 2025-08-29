use crate::{
    clone::{clone, force_clone},
    fetch::config::Config,
    file::path::sanitize_dir,
    regex::extract_path,
};
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use console::{style, Emoji};
use dialoguer::Confirm;
use indicatif::HumanDuration;
use std::{fs, time::Instant};

static FINISH: Emoji<'_, '_> = Emoji("🚀", "🚀");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "🔥");

pub async fn direct_clone(matches: &ArgMatches, url: &String) -> Result<()> {
    let mode = matches.get_one::<String>("mode").unwrap();
    let branch = matches.get_one::<String>("branch");
    let force = matches.get_flag("force");
    let keep_history = matches.get_flag("keep-history");

    let (_, repo_dir) = extract_path(url).ok_or_else(|| eyre!("Invalid URL"))?;
    let dir_str = match matches.get_one::<String>("dir") {
        Some(dir) => dir,
        None => &repo_dir.to_string(),
    };

    let dir_path = sanitize_dir(dir_str)?;
    let dir_string = dir_path
        .to_str()
        .ok_or_else(|| eyre!("Invalid directory name: contains non-UTF-8 characters"))?
        .to_string();

    let config = Config::from(&dir_string, mode.into(), force, keep_history, branch);
    let started = Instant::now();

    if dir_path.exists() {
        let mut empty = fs::read_dir(&dir_path)?;
        if empty.next().is_some() {
            let force = config.force
                || Confirm::new()
                    .with_prompt("Do you want to overwrite existing files?")
                    .default(false)
                    .interact()
                    .map_err(|e| eyre!("Failed to interact with user: {}", e))?;
            if force {
                force_clone(&url.to_string(), &dir_string, &config).await?;
            } else {
                println!("{}", style("❌ Directory is not empty").red().bold());
                return Err(eyre!("Directory is not empty"));
            }
        } else {
            clone(&url.to_string(), &config).await?;
        }
    } else {
        clone(&url.to_string(), &config).await?;
    }

    println!("{} Done in {}", FINISH, HumanDuration(started.elapsed()));
    println!(
        "{} {}",
        FIRE,
        style("The Repo is prepared and ready to use!")
            .green()
            .bold()
    );
    Ok(())
}
