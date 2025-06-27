pub mod cmd;
pub mod file;
pub mod models;
pub mod regex;

use cmd::git::Git;
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use models::{mode::Mode, site::Site};
use regex::{extract_path, is_github_url};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};
use tar::Archive;

#[derive(Debug, Default)]
pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub site: Option<&'a Site>,
    pub mode: Option<&'a Mode>,
    pub force: bool,
}

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
    println!("{} Regit: {}", "🔄", style(url).bold());

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
        .execute("clone")
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
    println!("Hello I'm working on the tar mode");

    let host = match config.site {
        Some(site) => site.to_str(),
        None => {
            let site_options = vec![Site::Github.to_str(), Site::Gitlab.to_str()];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick the site you want to clone from (because you choose tar mode)")
                .default(0)
                .items(&site_options)
                .interact()
                .map_err(|e| e.to_string())?;
            site_options[selection]
        }
    };
    let (owner, repo) = extract_path(url).unwrap();
    println!("Let's clone the repo from {host}: {owner}/{repo} to {dir}");

    let output = Command::new("git")
        .args(["ls-remote", url])
        .output()
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
    println!("Hash: {:?}", hash);
    // println!("Remote: {:?}", remote);
    let archive_url = if host == "gitlab" {
        format!("{}/repository/archive.tar.gz?ref={}", url, hash)
    } else if host == "bitbucket" {
        format!("{}/get/{}.tar.gz", url, hash)
    } else {
        format!("{}/archive/{}.tar.gz", url, hash)
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
    let temp_file = download_file(&archive_url, dir, &pb).unwrap();
    pb.set_message("Extracting files...");
    extract_archive(&temp_file, dir)?;
    std::fs::remove_file(temp_file).map_err(|e| e.to_string())?;
    pb.finish_and_clear();
    println!("{} Repository prepared!", style("✨").cyan().bold());

    Ok(())
}

fn download_file(url: &str, dir: &str, pb: &ProgressBar) -> Result<PathBuf, String> {
    pb.set_message("🚀 Downloading...");

    // path of temp directory in OS
    let temp_dir = std::env::temp_dir();
    let file_name = format!(
        "{}_{}.tar.gz",
        dir,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let temp_file_path = temp_dir.join(file_name);

    let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Dowload Error: {}", response.status()));
    }

    let bytes = response.bytes().map_err(|e| e.to_string())?;
    let mut file = File::create(&temp_file_path).expect("Failed to create temp file");
    Write::write_all(&mut file, &bytes).expect("Failed to write temp file");
    Ok(temp_file_path)
}

fn extract_archive(temp_file_path: &PathBuf, dir: &str) -> std::io::Result<()> {
    let tar_gz = File::open(temp_file_path).expect("Failed to open the temp file");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dir)?;

    todo!("Remove the top level directory");
}
