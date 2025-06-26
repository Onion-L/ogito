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
        match s {
            "github" => Some(Site::Github),
            "gitlab" => Some(Site::Gitlab),
            _ => None,
        }
    }
}
