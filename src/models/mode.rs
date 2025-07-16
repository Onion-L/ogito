#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Git,
    Tar,
    Uninitialized,
}

impl From<&String> for Mode {
    fn from(value: &String) -> Mode {
        match value.as_str() {
            "git" => Mode::Git,
            "tar" => Mode::Tar,
            _ => Mode::Uninitialized,
        }
    }
}
