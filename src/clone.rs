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
use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use git2::build::RepoBuilder;
// use dialoguer::{Select, theme::ColorfulTheme};
use color_eyre::{Result, eyre::eyre};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, path::Path, thread, time::Duration};

#[derive(Debug, Clone)]
struct RemoteRef {
    hash: String,
    name: String,
}

pub async fn clone<'a>(url: &str, config: &Config<'a>) -> Result<()> {
    let dir = config.dir;
    let keep_history = config.keep_history;

    match config.mode {
        Mode::Git => git_clone(url, config)?,
        Mode::Tar => tar_clone(url, config, &dir, keep_history).await?,
        _ => return Err(eyre!("Invalid mode: {:?}", config.mode)),
    }
    Ok(())
}

pub async fn force_clone<'a>(url: &str, dir: &str, config: &Config<'a>) -> Result<()> {
    fs::remove_dir_all(dir)?;
    clone(url, config).await?;
    Ok(())
}

fn git_clone(url: &str, config: &Config) -> Result<()> {
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
            pb_clone.set_message("ogito🍸...");
        }
    });

    let dir_path = Path::new(config.dir);
    let mut builder = RepoBuilder::new();

    let status = match config.branch {
        Some(branch) => {
            pb.finish_and_clear();
            if branch != "INTERACTIVE" {
                builder.branch(branch);
            } else {
                let refs = get_remote_refs(&url)?;
                let binding = refs.clone();
                let refs_name: Vec<String> = binding
                    .into_iter()
                    .map(|r| r.name)
                    .filter(|r| r != "HEAD")
                    .collect();

                let branch: usize = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pick the branch you want to clone")
                    .default(0)
                    .items(&refs_name)
                    .interact()
                    .map_err(|e| eyre!("Failed to interact with user: {}", e))?;

                let branch_name = &refs[branch + 1].name.split('/').last().unwrap();
                builder.branch(branch_name);
            }
            builder.clone(url, dir_path)
        }
        None => builder.clone(url, dir_path),
    };

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
    if !config.keep_history {
        let git_dir = dir_path.join(".git");
        fs::remove_dir_all(git_dir)?;
    }

    let _ = handle.join();
    Ok(())
}

async fn tar_clone<'a>(
    url: &str,
    config: &Config<'a>,
    dir: &str,
    keep_history: bool,
) -> Result<()> {
    if keep_history {
        let use_git = Confirm::new()
            .with_prompt("Tar mode does not support keep history, do you want to use git instead?")
            .default(false)
            .interact()
            .map_err(|e| eyre!("Failed to interact with user: {}", e))?;
        if use_git {
            git_clone(url, config)?;
        } else {
            return Err(eyre!(
                "Tar mode does not support keep history, please use git instead"
            ));
        }
    }
    let (owner, repo) = extract_path(url).unwrap();
    let host = extract_host(url);

    // TODO select from different commits
    let refs = get_remote_refs(&url)?;
    let hash = &refs[0].hash;
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

fn get_remote_refs(url: &str) -> Result<Vec<RemoteRef>> {
    let mut git = Git::new();
    let output = git
        .args(vec![url])
        .ls_remote()
        .map_err(|e| eyre!("Failed to execute git ls-remote: {}", e))?;

    let stdout = String::from_utf8(output.stdout).unwrap();

    let refs = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() == 2 {
                Some(RemoteRef {
                    hash: parts[0].to_string(),
                    name: parts[1].to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<RemoteRef>>();

    Ok(refs)
}
