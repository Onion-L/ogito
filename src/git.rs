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
