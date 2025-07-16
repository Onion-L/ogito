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
