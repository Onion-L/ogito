pub mod file;
pub mod models;
pub mod utils;

use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use models::{mode::Mode, site::Site};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use utils::is_github_url;

#[derive(Debug, Default)]
pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub site: Option<&'a Site>,
    pub mode: Option<&'a Mode>,
    pub force: bool,
}

pub fn force_clone(url: &str, dir: &str, config: &Config) -> Result<(), String> {
    fs::remove_dir_all(dir).unwrap();
    clone(url, config).unwrap();
    Ok(())
}

pub fn clone(url: &str, config: &Config) -> Result<(), String> {
    dbg!(&config);
    let dir = config.dir.unwrap().to_string();
    if let Some(mode) = config.mode {
        match mode {
            Mode::Git => run_git_clone(url, &dir)?,
            Mode::Tar => run_tar_clone(url, &dir, config)?,
        }
    }

    Ok(())
}

fn run_git_clone(url: &str, dir: &str) -> Result<(), String> {
    if !is_github_url(url) {
        return Err("The source is not a Github URL".to_string());
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
            thread::sleep(Duration::from_millis(200));
            pb_clone.set_message("🚀 Downloading...");
        }
    });

    let clone_status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Failed to execute git clone");

    pb.finish_and_clear();

    if !clone_status.success() {
        println!("{} Git clone failed!", style("❌").red().bold());
        return Err("Failed to execute git clone".to_string());
    }

    let git_dir = Path::new(dir).join(".git");
    fs::remove_dir_all(git_dir).expect("Failed to remove .git directory");
    println!("{} Repository prepared!", style("✨").cyan().bold());

    let _ = handle.join();
    Ok(())
}

fn run_tar_clone(url: &str, dir: &str, config: &Config) -> Result<(), String> {
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
    let (owner, repo) = extract_path_regex(url).unwrap();
    println!("Let's clone the repo from {host}: {owner}/{repo} to {dir}");

    Ok(())
}

fn extract_path_regex(url: &str) -> Option<(&str, &str)> {
    let re = regex::Regex::new(r"https?://[^/]+/(.*)").ok()?;
    let path = re.captures(url)?.get(1).map(|m| m.as_str())?;
    let (owner, repo) = (path.split("/").next()?, path.split("/").last()?);
    Some((owner, repo))
}
