use crate::{
    fetch::config::Config,
    file::{download_file, extract_archive},
    git::Git,
    models::Mode,
    models::Site,
    regex::extract_host,
    regex::{extract_path, is_valid_url},
};
use console::style;
use git2::Repository;
// use dialoguer::{Select, theme::ColorfulTheme};
use color_eyre::{Result, eyre::eyre};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, path::Path, thread, time::Duration};

pub async fn clone<'a>(url: &str, config: &Config<'a>) -> Result<()> {
    let dir = config.dir;

    match config.mode {
        Mode::Git => git_clone(url, &dir)?,
        Mode::Tar => tar_clone(url, &dir).await?,
        _ => return Err(eyre!("Invalid mode: {:?}", config.mode)),
    }
    Ok(())
}

pub async fn force_clone<'a>(url: &str, dir: &str, config: &Config<'a>) -> Result<()> {
    fs::remove_dir_all(dir)?;
    clone(url, config).await?;
    Ok(())
}

fn git_clone(url: &str, dir: &str) -> Result<()> {
    if !is_valid_url(url) {
        return Err(eyre!("The source is not a valid URL"));
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

    let status = Repository::clone(url, dir);
    pb.finish_and_clear();

    let repo = match status {
        Ok(repo) => {
            println!("{} Repository cloned!", style("✨").cyan().bold());
            repo
        }
        Err(e) => {
            println!("{} Git clone failed!", style("❌").red().bold());
            return Err(eyre!("Git clone failed: {}", e));
        }
    };
    drop(repo);

    let git_dir = Path::new(dir).join(".git");
    fs::remove_dir_all(git_dir)?;
    println!("{} Repository prepared!", style("✨").cyan().bold());

    let _ = handle.join();
    Ok(())
}

async fn tar_clone(url: &str, dir: &str) -> Result<()> {
    let (owner, repo) = extract_path(url).unwrap();
    let host = extract_host(url);
    let mut git = Git::new();
    let output = git
        .args(vec![url])
        .ls_remote()
        .map_err(|e| eyre!("Failed to execute git ls-remote: {}", e))?;

    let stdout = String::from_utf8(output.stdout).unwrap();
    let binding = stdout.clone();
    let hash_list = binding.split("\n").collect::<Vec<&str>>();

    // TODO select from different commits
    /*let hash: Vec<&str> = stdout
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
      */

    let hash = hash_list[0].split("\t").collect::<Vec<&str>>()[0];

    let archive_url = match host.map(Site::from) {
        Some(Site::Gitlab) => format!(
            "https://gitlab.com/{}/{}/repository/archive.tar.gz?ref={}",
            owner, repo, hash
        ),
        Some(Site::Github) => format!(
            "https://github.com/{}/{}/archive/{}.tar.gz",
            owner, repo, hash
        ),
        _ => String::new(),
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

    let temp_file = download_file(&archive_url, dir, &pb).await?;

    pb.set_message("Extracting files...");
    extract_archive(&temp_file, dir)?;

    std::fs::remove_file(temp_file)?;
    pb.finish_and_clear();
    println!("{} Repository prepared!", style("✨").cyan().bold());
    let _ = handle.join();
    Ok(())
}
