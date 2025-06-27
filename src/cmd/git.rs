use std::{
    io::Error,
    process::{Command, ExitStatus, Stdio},
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

    pub fn execute(&self, arg: &str) -> Result<ExitStatus, Error> {
        Command::new(self.cmd)
            .arg(arg)
            .args(&self.args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    }
}
