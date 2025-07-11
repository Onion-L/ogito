pub mod cmd;
pub mod code;
pub mod fetch;
pub mod file;
pub mod models;
pub mod regex;
pub mod tui;

use crate::file::{download_file, extract_archive};
use cmd::git::Git;
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use fetch::config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use models::{mode::Mode, site::Site};
use regex::{extract_path, is_github_url};
use std::{fs, path::Path, thread, time::Duration};

pub fn clone(url: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let dir = config.dir.unwrap().to_string();
    if let Some(mode) = config.mode {
        match mode {
            Mode::Git => git_clone(url, &dir)?,
            Mode::Tar => tar_clone(url, &dir, config)?,
        }
    }

    Ok(())
}

pub fn force_clone(
    url: &str,
    dir: &str,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir_all(dir).unwrap();
    clone(url, config).unwrap();
    Ok(())
}

fn git_clone(url: &str, dir: &str) -> Result<(), std::io::Error> {
    if !is_github_url(url) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The source is  not a Github URL",
        ));
    }
    println!("{} ogito: {}", "🔄", style(url).bold());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Downloading repository...");

    let pb_clone = pb.clone();
    let handle = thread::spawn(move || {
        let messages = [
            "🔗 Connecting to remote server...",
            "🔍 Getting repository information...",
            "📥 Downloading files...",
            "🔥 Checking file integrity...",
        ];

        for msg in messages {
            thread::sleep(Duration::from_millis(500));
            pb_clone.set_message(msg);
        }

        while !pb_clone.is_finished() {
            thread::sleep(Duration::from_millis(500));
            pb_clone.set_message("🚀 Downloading...");
        }
    });

    let mut git = Git::new();
    let clone_status = git
        .args(vec![url, dir])
        .clone()
        .expect("Failed to execute git clone");

    pb.finish_and_clear();

    if !clone_status.success() {
        println!("{} Git clone failed!", style("❌").red().bold());
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute git clone",
        ));
    }

    let git_dir = Path::new(dir).join(".git");
    fs::remove_dir_all(git_dir).expect("Failed to remove .git directory");
    println!("{} Repository prepared!", style("✨").cyan().bold());

    let _ = handle.join();
    Ok(())
}

fn tar_clone(url: &str, dir: &str, config: &Config<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let host = match config.site {
        Some(site) => site,
        None => {
            let site_options = vec![Site::Github.to_str(), Site::Gitlab.to_str()];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick the site you want to clone from (because you choose tar mode)")
                .default(0)
                .items(&site_options)
                .interact()
                .map_err(|e| e.to_string())?;
            Site::from_str(site_options[selection]).unwrap()
        }
    };
    let (owner, repo) = extract_path(url).unwrap();

    let mut git = Git::new();
    let output = git
        .args(vec![url])
        .ls_remote()
        .expect("Failed to execute git ls-remote");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let binding = stdout.clone();
    let hash_list = binding.split("\n").collect::<Vec<&str>>();

    let hash: Vec<&str> = stdout
        .split("\n")
        .filter_map(|line| {
            let parts: Vec<&str> = line.split("\t").collect();
            if parts.len() == 2 && !parts[1].is_empty() {
                Some(parts[1])
            } else {
                None
            }
        })
        .collect();

    let branch: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick the branch you want to clone")
        .default(0)
        .items(&hash)
        .interact()
        .map_err(|e| e.to_string())?;

    let hash = hash_list[branch].split("\t").collect::<Vec<&str>>()[0];
    let archive_url = if host == Site::Gitlab {
        format!(
            "https://gitlab.com/{}/{}/repository/archive.tar.gz?ref={}",
            owner, repo, hash
        )
    } else {
        format!(
            "https://github.com/{}/{}/archive/{}.tar.gz",
            owner, repo, hash
        )
    };

    println!(
        "{} Downloading archive from: {}",
        "📦",
        style(&archive_url).bold()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Downloading archive...");

    let pb_clone = pb.clone();
    let handle = thread::spawn(move || {
        let messages = [
            "🔗 Connecting to remote server...",
            "🔍 Getting repository information...",
            "📥 Downloading files...",
            "🔥 Checking file integrity...",
        ];

        for msg in messages {
            thread::sleep(Duration::from_millis(500));
            pb_clone.set_message(msg);
        }

        while !pb_clone.is_finished() {
            thread::sleep(Duration::from_millis(500));
            pb_clone.set_message("🚀 Downloading...");
        }
    });

    let temp_file = download_file(&archive_url, dir, &pb).unwrap();

    pb.set_message("Extracting files...");
    extract_archive(&temp_file, dir)?;

    std::fs::remove_file(temp_file).map_err(|e| e.to_string())?;
    pb.finish_and_clear();
    println!("{} Repository prepared!", style("✨").cyan().bold());
    let _ = handle.join();
    Ok(())
}
