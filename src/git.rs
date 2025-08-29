use color_eyre::{eyre::eyre, Result};
use std::{
    io::Error,
    process::{Command, Output},
};

pub struct Git<'a> {
    pub cmd: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Git<'a> {
    pub fn new() -> Self {
        Self {
            cmd: "git",
            args: vec![],
        }
    }

    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Self {
        self.args = args;
        self
    }

    pub fn ls_remote(&self) -> Result<Output, Error> {
        Command::new(self.cmd)
            .arg("ls-remote")
            .args(&self.args)
            .output()
    }
}

#[derive(Debug, Clone)]
pub struct RemoteRef {
    pub hash: String,
    pub name: String,
}

pub fn get_remote_refs(url: &str) -> Result<Vec<RemoteRef>> {
    let mut git = Git::new();
    let output = git.args(vec![url]).ls_remote()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("git ls-remote failed for {}: {}", url, stderr.trim()));
    }

    let stdout = String::from_utf8(output.stdout)?;

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

    if refs.is_empty() {
        return Err(eyre!("No remote references found for {}", url));
    }

    Ok(refs)
}

#[cfg(test)]
mod tests {
    use super::Git;

    #[test]
    fn test_git_args() {
        let mut git = Git::new();
        git.args(vec!["https://github.com/owner/repo.git", "/tmp/repo"]);
        assert_eq!(git.cmd, "git");
        assert_eq!(
            git.args,
            vec!["https://github.com/owner/repo.git", "/tmp/repo"]
        );
    }
}
