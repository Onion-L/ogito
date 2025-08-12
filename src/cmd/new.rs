use crate::{
    clone::{clone, force_clone},
    fetch::config::Config,
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

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("url")
        .ok_or_else(|| eyre!("URL is required"))?;
    let mode = matches.get_one::<String>("mode").unwrap();
    let branch = matches.get_one::<String>("branch");
    let force = matches.get_flag("force");
    let keep_history = matches.get_flag("keep-history");

    let (_, repo_dir) = extract_path(url).ok_or_else(|| eyre!("Invalid URL"))?;
    let dir = match matches.get_one::<String>("dir") {
        Some(dir) => dir,
        None => &repo_dir.to_string(),
    };

    let config = Config::from(dir, mode.into(), force, keep_history, branch);
    let started = Instant::now();

    if !fs::metadata(dir).is_ok() {
        clone(&url.to_string(), &config).await?;
    } else {
        let mut empty = fs::read_dir(dir)?;
        if empty.next().is_some() {
            let force = config.force
                || Confirm::new()
                    .with_prompt("Do you want to overwrite existing files?")
                    .default(false)
                    .interact()
                    .map_err(|e| eyre!("Failed to interact with user: {}", e))?;
            if force {
                force_clone(&url.to_string(), dir, &config).await?;
            } else {
                println!("{}", style("❌ Directory is not empty").red().bold());
                return Err(eyre!("Directory is not empty"));
            }
        } else {
            clone(&url.to_string(), &config).await?;
        }
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
