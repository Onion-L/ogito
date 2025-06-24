use clap::{ArgAction, arg, command};
use console::Emoji;
use console::style;
use dialoguer::Confirm;
use indicatif::HumanDuration;
use regit::{Config, Mode};
use std::{fs, time::Instant};

static FINISH: Emoji<'_, '_> = Emoji("🚀 ", "✅ ");

fn main() {
    let matches = command!()
        .about("A simple git clone manager")
        .arg(arg!([url] "the link to the source file"))
        .arg(arg!(-r --repo <REPO> "the repository name, e.g. 'user/repo'").required(false))
        .arg(arg!(-d --dir <DIRNAME> "the directory name").required(true))
        .arg(
            arg!( -s --site <SITE> "Sets the site or use Github by default")
                .default_value("github"),
        )
        .arg(
            arg!(-m --mode <MODE> "the mode of the operation")
                .required(false)
                .default_value(Mode::Git.as_str()),
        )
        .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue))
        .get_matches();

    let started = Instant::now();

    let dir = matches
        .get_one::<String>("dir")
        .expect("Directory name is required");

    let url = matches.get_one::<String>("url").unwrap();

    let config = Config {
        repo: matches.get_one::<String>("repo"),
        dir: matches.get_one::<String>("dir"),
        site: matches.get_one::<String>("site"),
        force: matches.get_flag("force"),
    };

    // check if the directory exists
    if !fs::metadata(dir).is_ok() {
        regit::clone(&url.to_string(), &config).unwrap();
    } else {
        let mut empty = fs::read_dir(dir).unwrap();
        if empty.next().is_some() {
            if !matches.get_flag("force") {
                let force = Confirm::new()
                    .with_prompt("Overwrite existing files?")
                    .default(false)
                    .interact()
                    .unwrap();
                if force {
                    regit::force_clone(&url.to_string(), dir, &config).unwrap();
                } else {
                    println!("{}", style("❌ Directory is not empty").red().bold());
                    return;
                }
            } else {
                regit::force_clone(&url.to_string(), dir, &config).unwrap();
            }
        } else {
            regit::clone(&url.to_string(), &config).unwrap();
        }
    }

    println!("{}Done in {}", FINISH, HumanDuration(started.elapsed()));
}
