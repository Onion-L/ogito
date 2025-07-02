use clap::{ArgAction, arg, command};
use color_eyre::{Result, eyre::eyre};
use console::Emoji;
use console::style;
use dialoguer::Confirm;
use indicatif::HumanDuration;
use std::{fs, time::Instant};

use regit::fetch::config::Config;
use regit::file::get_repo;
use regit::models::mode::Mode;
use regit::models::site::Site;
use regit::regex::extract_path;
use regit::tui::app::App;

static FINISH: Emoji<'_, '_> = Emoji("🚀", "🚀");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "🔥");

fn main() -> Result<()> {
    let matches = command!()
        .about("A simple git clone manager")
        .arg(arg!([url] "the link to the source file"))
        .arg(arg!(-r --repo <REPO> "the repository name, e.g. 'user/repo'").required(false))
        .arg(arg!(-d --dir <DIRNAME> "the directory name").required(false))
        .arg(
            arg!( -s --site <SITE> "Sets the site or use Github by default")
                .value_parser(Site::from_str),
        )
        .arg(
            arg!(-m --mode <MODE> "the mode of the operation")
                .required(false)
                .value_parser(Mode::from_str)
                .default_value(Mode::Git.to_str()),
        )
        .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue))
        .get_matches();

    let url = matches
        .get_one::<String>("url")
        .expect("URL is required. regit <URL>");
    let repo = matches.get_one::<String>("repo");
    let site = matches.get_one::<Site>("site");
    let mode = matches.get_one::<Mode>("mode");
    let force = matches.get_flag("force");

    let (_, repo_dir) = extract_path(url).unwrap();
    let dir = match matches.get_one::<String>("dir") {
        Some(dir) => dir,
        None => &repo_dir.to_string(),
    };

    let config = Config::from(repo, Some(dir), site, mode, force);
    let started = Instant::now();
    // check if the directory exists
    if !fs::metadata(dir).is_ok() {
        regit::clone(&url.to_string(), &config).unwrap();
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
                regit::force_clone(&url.to_string(), dir, &config).unwrap();
            } else {
                println!("{}", style("❌ Directory is not empty").red().bold());
                return Err(eyre!("Directory is not empty"));
            }
        } else {
            regit::clone(&url.to_string(), &config).unwrap();
        }
    }

    println!("{} Done in {}", FINISH, HumanDuration(started.elapsed()));

    let tui = Confirm::new()
        .with_prompt("💻 Open TUI to manage the files?")
        .default(false)
        .interact()
        .unwrap();
    if tui {
        let (dirs, files) = get_repo(dir).unwrap();
        let _ = App::from(dirs, files);
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
