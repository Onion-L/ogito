#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Git,
    Tar,
    Unknown,
}

impl From<&String> for Mode {
    fn from(value: &String) -> Mode {
        match value.as_str() {
            "git" => Mode::Git,
            "tar" => Mode::Tar,
            _ => Mode::Unknown,
        }
    }
}
