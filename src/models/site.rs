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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Site::Github.to_str(), "github");
        assert_eq!(Site::Gitlab.to_str(), "gitlab");
    }

    #[test]
    fn test_from_str_option() {
        assert_eq!(Site::from_str("github"), Some(Site::Github));
        assert_eq!(Site::from_str("gitlab"), Some(Site::Gitlab));
        assert_eq!(Site::from_str("gitee"), None);
    }
}
