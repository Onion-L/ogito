use regex::Regex;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

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

pub fn regit(url: &str, config: &Config) -> Result<(), String> {
    dbg!(&config.force, &config.dir, &config.repo, &config.site);
    let dir = config.dir.unwrap().to_string();
    if is_github_url(url) {
        run_git_clone(url, &dir)?;
    } else {
        return Err("The source is not a Github URL".to_string());
    }
    Ok(())
}

fn is_github_url(input: &str) -> bool {
    let re = Regex::new(r"^(?:https://)?github\.com/([^/]+)/([^/]+?)(?:\.git)?$").unwrap();
    re.is_match(input)
}

fn run_git_clone(url: &str, dir: &str) -> Result<(), String> {
    let clone_status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Failed to execute git clone");

    if !clone_status.success() {
        return Err("Failed to execute git clone".to_string());
    }

    let git_dir = Path::new(dir).join(".git");
    fs::remove_dir_all(git_dir).expect("Failed to remove .git directory");

    Ok(())
}
