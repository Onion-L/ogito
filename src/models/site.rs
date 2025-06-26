#[derive(Debug, Clone)]
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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "github" => Ok(Site::Github),
            "gitlab" => Ok(Site::Gitlab),
            _ => Err(format!("Invalid site: {}", s)),
        }
    }
}
