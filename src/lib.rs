use std::{fs, path::Path, process::Command};

use regex::Regex;

pub fn regit(url: &str, dir: &str) -> Result<(), String> {
    if is_github_url(url) {
        run_git_clone(url, dir)?;
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
        .status()
        .expect("Failed to execute git clone");

    if !clone_status.success() {
        return Err("Failed to execute git clone".to_string());
    }

    let git_dir = Path::new(dir).join(".git");
    fs::remove_dir_all(git_dir).expect("Failed to remove .git directory");

    Ok(())
}
