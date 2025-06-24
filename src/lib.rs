pub mod utils;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use utils::is_github_url;

pub enum Site {
    Github,
    Gitlab,
    Bitbucket,
    Gitee,
    Gitcode,
}

pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub site: Option<&'a String>,
    pub force: bool,
}

pub enum Mode {
    Git,
    Tar,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Git => "git",
            Mode::Tar => "tar",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "git" => Some(Mode::Git),
            "tar" => Some(Mode::Tar),
            _ => None,
        }
    }
}

pub fn force_clone(url: &str, dir: &str, config: &Config) -> Result<(), String> {
    fs::remove_dir_all(dir).unwrap();
    clone(url, config).unwrap();
    Ok(())
}

pub fn clone(url: &str, config: &Config) -> Result<(), String> {
    let dir = config.dir.unwrap().to_string();
    if is_github_url(url) {
        run_git_clone(url, &dir)?;
    } else {
        return Err("The source is not a Github URL".to_string());
    }

    Ok(())
}

fn run_git_clone(url: &str, dir: &str) -> Result<(), String> {
    println!("{} Regit: {}", "🔄", style(url).bold(),);

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
