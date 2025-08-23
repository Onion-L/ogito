use crate::file;
use crate::progress::create_spinner;
use crate::{
    fetch::config::Config,
    file::cache::CacheMetadata,
    git::get_remote_refs,
    mode::{Mode, Site},
    regex::{extract_host, extract_path, is_valid_url},
};
use color_eyre::{eyre::eyre, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use git2::build::RepoBuilder;
use std::{fs, path::Path};

pub async fn clone<'a>(url: &str, config: &Config<'a>) -> Result<()> {
    match config.mode {
        Mode::Git => git_clone(url, config)?,
        Mode::Tar => tar_clone(url, config).await?,
        _ => return Err(eyre!("Invalid mode: {:?}", config.mode)),
    }
    println!("{} Repository is ready!", style("✨").cyan().bold());

    Ok(())
}

pub async fn force_clone<'a>(url: &str, dir: &str, config: &Config<'a>) -> Result<()> {
    fs::remove_dir_all(dir)?;
    clone(url, config).await?;
    Ok(())
}

fn git_clone(url: &str, config: &Config) -> Result<()> {
    if !is_valid_url(url)? {
        return Err(eyre!("The source is not a valid URL"));
    }
    println!("🍸 ogito: {}", style(url).bold());

    let pb = create_spinner("🔗 Connecting to remote server...");
    pb.set_message("📥 Cloning repository...");

    let dir_path = Path::new(config.dir);
    let mut builder = RepoBuilder::new();

    let status = match config.branch {
        Some(branch) => {
            pb.finish_and_clear();
            if branch != "INTERACTIVE" {
                builder.branch(branch);
            } else {
                let refs = get_remote_refs(url)?;
                let binding = refs.clone();
                let refs_name: Vec<String> = binding
                    .into_iter()
                    .filter(|r| r.name.starts_with("refs/heads/"))
                    .map(|r| r.name.replace("refs/heads/", ""))
                    .collect();

                let branch: usize = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "Pick the branch you want to clone (if you want to clone a tag, please use the tar mode)",
                    )
                    .default(0)
                    .items(&refs_name)
                    .interact()
                    .map_err(|e| eyre!("Failed to interact with user: {}", e))?;

                let branch_name = &refs[branch + 1].name.replace("refs/heads/", "");
                builder.branch(branch_name);
            }
            builder.clone(url, dir_path)
        }
        None => builder.clone(url, dir_path),
    };

    pb.finish_with_message("✅ Repository cloned successfully");

    let repo = match status {
        Ok(repo) => repo,
        Err(e) => {
            return Err(eyre!("❌ Git clone failed: {}", e));
        }
    };
    drop(repo);

    if !config.keep_history {
        let git_dir = dir_path.join(".git");
        fs::remove_dir_all(git_dir)?;
    }

    Ok(())
}

async fn tar_clone<'a>(url: &str, config: &Config<'a>) -> Result<()> {
    if config.keep_history {
        let use_git = Confirm::new()
            .with_prompt("Tar mode does not support keep history, do you want to use git instead?")
            .default(false)
            .interact()
            .map_err(|e| eyre!("Failed to interact with user: {}", e))?;
        if use_git {
            return git_clone(url, config);
        } else {
            return Err(eyre!(
                "Tar mode does not support keep history, please use git instead"
            ));
        }
    }

    let (owner, repo) = extract_path(url).ok_or_else(|| eyre!("Invalid URL"))?;
    let host = extract_host(url);

    let refs = get_remote_refs(url)?;
    let hash = match config.branch {
        Some(branch) => {
            if branch != "INTERACTIVE" {
                let remote_ref = refs
                    .iter()
                    .find(|r| {
                        r.name == format!("refs/heads/{branch}")
                            || r.name == format!("refs/tags/{branch}")
                    })
                    .ok_or_else(|| eyre!("Branch or tag '{}' not found.", branch))?;
                remote_ref.hash.clone()
            } else {
                let refs = get_remote_refs(url)?;
                let binding = refs.clone();

                let refs_name: Vec<String> = binding
                    .into_iter()
                    .filter(|r| {
                        r.name.starts_with("refs/heads/") || r.name.starts_with("refs/tags/")
                    })
                    .map(|r| r.name.replace("refs/heads/", "").replace("refs/tags/", ""))
                    .collect();

                let branch: usize = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pick the branch you want to clone")
                    .default(0)
                    .items(&refs_name)
                    .interact()
                    .map_err(|e| eyre!("Failed to interact with user: {}", e))?;

                refs[branch + 1].hash.clone()
            }
        }
        None => {
            let head_ref = refs.iter().find(|r| r.name == "HEAD").ok_or_else(|| {
                eyre!("No HEAD reference found, cannot determine default branch.")
            })?;
            head_ref.hash.clone()
        }
    };

    let cache_metadata = CacheMetadata::new(owner, repo, &hash);

    let archive_url = match host.map(Site::from) {
        Some(Site::Gitlab) => format!(
            "https://gitlab.com/{}/{}/repository/archive.tar.gz?ref={}",
            cache_metadata.owner, cache_metadata.repo, cache_metadata.hash
        ),
        Some(Site::Github) => format!(
            "https://github.com/{}/{}/archive/{}.tar.gz",
            cache_metadata.owner, cache_metadata.repo, cache_metadata.hash
        ),
        _ => String::new(),
    };

    println!(
        "📦 Downloading archive from: {}",
        style(&archive_url).bold()
    );

    let pb = create_spinner("📥 Downloading archive...");

    let dir = config.dir;
    let temp_file = file::download_file(&archive_url, &cache_metadata).await?;

    pb.set_message("🗜️ Extracting archive...");
    file::extract_archive(&temp_file, dir)?;

    pb.finish_and_clear();
    Ok(())
}
