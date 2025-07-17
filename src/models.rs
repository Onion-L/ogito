#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Site {
    Github,
    Gitlab,
    Unknown,
}

impl From<String> for Site {
    fn from(value: String) -> Site {
        match value.to_lowercase().as_str() {
            "github" => Site::Github,
            "gitlab" => Site::Gitlab,
            _ => Site::Unknown,
        }
    }
}

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
