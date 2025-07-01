use std::{
    io::Error,
    process::{Command, ExitStatus, Output, Stdio},
};

pub struct Git<'a> {
    cmd: &'a str,
    args: Vec<&'a str>,
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

    pub fn clone(&self) -> Result<ExitStatus, Error> {
        Command::new(self.cmd)
            .arg("clone")
            .args(&self.args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    }

    pub fn ls_remote(&self) -> Result<Output, Error> {
        Command::new(self.cmd)
            .arg("ls-remote")
            .args(&self.args)
            .output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
