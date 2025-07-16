use clap::{ArgAction, arg, command};
use color_eyre::{Result, eyre::eyre};
use console::Emoji;
use console::style;
use dialoguer::Confirm;
use indicatif::HumanDuration;
use std::ffi::OsString;
use std::{fs, time::Instant};

use ogito::fetch::config::Config;
use ogito::file::get_repo;
use ogito::regex::extract_path;
use ogito::tui::app::App;

static FINISH: Emoji<'_, '_> = Emoji("🚀", "🚀");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "🔥");

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .about("A simple git clone manager")
        .arg(arg!([url] "the link to the source file"))
        .arg(arg!(-r --repo <REPO> "the repository name, e.g. 'user/repo'").required(false))
        .arg(arg!(-d --dir <DIRNAME> "the directory name").required(false))
        .arg(
            arg!(-m --mode <MODE> "the mode of the operation")
                .required(false)
                .default_value("git"),
        )
        .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue))
        .get_matches();

    let url = matches
        .get_one::<String>("url")
        .expect("URL is required. ogito <URL>");
    let repo = matches.get_one::<String>("repo");
    let mode = matches.get_one::<String>("mode").unwrap();
    let force = matches.get_flag("force");

    let (_, repo_dir) = extract_path(url).unwrap();
    let dir = match matches.get_one::<String>("dir") {
        Some(dir) => dir,
        None => &repo_dir.to_string(),
    };

    let config = Config::from(repo, Some(dir), mode.into(), force);
    let started = Instant::now();
    // check if the directory exists
    if !fs::metadata(dir).is_ok() {
        ogito::clone(&url.to_string(), &config).await.unwrap();
    } else {
        let mut empty = fs::read_dir(dir).unwrap();
        if empty.next().is_some() {
            let force = matches.get_flag("force")
                || Confirm::new()
                    .with_prompt("Do you want to overwrite existing files?")
                    .default(false)
                    .interact()
                    .unwrap();
            if force {
                ogito::force_clone(&url.to_string(), dir, &config)
                    .await
                    .unwrap();
            } else {
                println!("{}", style("❌ Directory is not empty").red().bold());
                return Err(eyre!("Directory is not empty"));
            }
        } else {
            ogito::clone(&url.to_string(), &config).await.unwrap();
        }
    }

    println!("{} Done in {}", FINISH, HumanDuration(started.elapsed()));

    let tui = Confirm::new()
        .with_prompt("💻 Open TUI to manage the files?")
        .default(false)
        .interact()
        .unwrap();
    if tui {
        let mut terminal = ratatui::init();
        let dir_os = OsString::from(dir);
        let repo = get_repo(&dir_os).unwrap();
        let app = App::from(dir_os, repo);
        let _ = app.run(&mut terminal);
        ratatui::restore();
        println!("{}", style("TUI is cooking right now 🫕").bold().yellow());
    }

    println!(
        "{} {}",
        FIRE,
        style("The Repo is prepared and ready to use!")
            .green()
            .bold()
    );
    Ok(())
}
