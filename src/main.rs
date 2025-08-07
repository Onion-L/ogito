use clap::{Arg, ArgAction, arg, command};
use color_eyre::eyre::ContextCompat;
use color_eyre::{Result, eyre::eyre};
use console::Emoji;
use console::style;
use dialoguer::Confirm;
use indicatif::HumanDuration;
use ogito::clone::{clone, force_clone};
use ogito::fetch::config::Config;
// use ogito::file::get_repo;
// use ogito::tui::app::App;
// use std::ffi::OsString;
use ogito::regex::extract_path;
use std::{fs, time::Instant};

static FINISH: Emoji<'_, '_> = Emoji("🚀", "🚀");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "🔥");

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .about("A simple git clone manager")
        .arg(arg!([url] "the link to the source file"))
        .arg(arg!(-d --dir <DIRNAME> "the directory name"))
        .arg(
            arg!(-b --branch [BRANCH] "the branch to clone")
                .required(false)
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
        )
        .arg_required_else_help(true)
        .get_matches();

    let url = matches.get_one::<String>("url");
    if let Some(url) = url {
        let mode = matches.get_one::<String>("mode").unwrap();
        let branch = matches.get_one::<String>("branch");

        let force = matches.get_flag("force");
        let keep_history = matches.get_flag("keep-history");

        let (_, repo_dir) = extract_path(url).wrap_err("Invalid URL")?;
        let dir = match matches.get_one::<String>("dir") {
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
                let force = matches.get_flag("force")
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
    }

    // TODO A new TUI, a template manager not a file manager
    // let tui = Confirm::new()
    //     .with_prompt("💻 Open TUI to manage the files?")
    //     .default(false)
    //     .interact()
    //     .map_err(|e| eyre!("Failed to interact with user: {}", e))?;

    // if tui {
    //     let terminal = &mut ratatui::init();
    //     let path = fs::canonicalize(std::env::current_dir()?.join(dir))?;
    //     let repo = get_repo(&OsString::from(dir))?;
    //     App::from(path, repo).run(terminal)?;
    //     ratatui::restore();
    // }

    println!(
        "{} {}",
        FIRE,
        style("The Repo is prepared and ready to use!")
            .green()
            .bold()
    );

    Ok(())
}
