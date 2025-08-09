mod clone;
mod fetch;
mod file;
mod git;
mod models;
mod regex;

use crate::clone::{clone, force_clone};
use crate::fetch::config::Config;
use crate::regex::extract_path;
use clap::{Arg, ArgAction, Command, arg, command};
use color_eyre::eyre::ContextCompat;
use color_eyre::{Result, eyre::eyre};
use console::Emoji;
use console::style;
use dialoguer::Confirm;
use indicatif::HumanDuration;
use std::{fs, time::Instant};

static FINISH: Emoji<'_, '_> = Emoji("🚀", "🚀");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "🔥");

#[tokio::main]
async fn main() -> Result<()> {
    let ogito = command!()
        .about("A simple git clone manager")
        .subcommand(
            Command::new("new")
                .about("Create a new template")
                .arg(arg!([url] "the link to the source file").required(true))
                .arg(arg!(-d --dir <DIRNAME> "the directory name"))
                .arg(
                    arg!(-b --branch [BRANCH] "the branch to clone")
                        .require_equals(true)
                        .num_args(0..=1)
                        .default_missing_value("INTERACTIVE"),
                )
                .arg(arg!(-m --mode <MODE> "the mode of the operation").default_value("git"))
                .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue))
                .arg(
                    Arg::new("keep-history")
                        .short('H')
                        .long("keep-history")
                        .help("keep the history of the repository")
                        .action(ArgAction::SetTrue),
                ),
        )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(ogito_new) = ogito.subcommand_matches("new") {
        let url = ogito_new
            .get_one::<String>("url")
            .ok_or_else(|| eyre!("URL is required"))?;
        let mode = ogito_new.get_one::<String>("mode").unwrap();
        let branch = ogito_new.get_one::<String>("branch");

        let force = ogito_new.get_flag("force");
        let keep_history = ogito_new.get_flag("keep-history");

        let (_, repo_dir) = extract_path(url).wrap_err("Invalid URL")?;
        let dir = match ogito_new.get_one::<String>("dir") {
            Some(dir) => dir,
            None => &repo_dir.to_string(),
        };

        let config = Config::from(dir, mode.into(), force, keep_history, branch);
        let started = Instant::now();
        // check if the directory exists
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
    }
    // TODO A new TUI, a template manager not a file manager

    Ok(())
}
