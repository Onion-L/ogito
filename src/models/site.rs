#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Site::Github.to_str(), "github");
        assert_eq!(Site::Gitlab.to_str(), "gitlab");
    }

    #[test]
    fn test_from_str_valid() {
        assert_eq!(Site::from_str("github"), Ok(Site::Github));
        assert_eq!(Site::from_str("gitlab"), Ok(Site::Gitlab));
    }

    #[test]
    fn test_from_str_invalid() {
        let err = Site::from_str("gitee");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), "Invalid site: gitee");
    }
}
