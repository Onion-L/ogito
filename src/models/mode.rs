#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Git,
    Tar,
}

impl Mode {
    pub fn to_str(&self) -> &'static str {
        match self {
            Mode::Git => "git",
            Mode::Tar => "tar",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "git" => Ok(Mode::Git),
            "tar" => Ok(Mode::Tar),
            _ => Err(format!("Invalid mode: {}", s)),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn mode_to_str() {
        assert_eq!(Mode::Git.to_str(), "git");
        assert_eq!(Mode::Tar.to_str(), "tar");
    }

    #[test]
    fn mode_from_str_valid() {
        assert_eq!(Mode::from_str("git"), Ok(Mode::Git));
        assert_eq!(Mode::from_str("tar"), Ok(Mode::Tar));
    }

    #[test]
    fn mode_from_str_invalid() {
        let err = Mode::from_str("zip");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), "Invalid mode: zip");
    }
}
