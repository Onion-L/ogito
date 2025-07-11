#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Site {
    Github,
    Gitlab,
}

impl Site {
    pub fn to_str(&self) -> &'static str {
        match self {
            Site::Github => "github",
            Site::Gitlab => "gitlab",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "github" => Some(Site::Github),
            "gitlab" => Some(Site::Gitlab),
            _ => None,
        }
    }
}
