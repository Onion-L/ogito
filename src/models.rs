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

#[cfg(test)]
mod tests {
    use super::Mode;

    #[test]
    fn mode_from_string() {
        let git_str = String::from("git");
        let tar_str = String::from("tar");
        let invalid_str = String::from("zip");

        assert_eq!(Mode::from(&git_str), Mode::Git);
        assert_eq!(Mode::from(&tar_str), Mode::Tar);
        assert_eq!(Mode::from(&invalid_str), Mode::Unknown);
    }

    #[test]
    fn mode_into() {
        let git_str = String::from("git");
        let tar_str = String::from("tar");
        let invalid_str = String::from("zip");

        let git_mode: Mode = (&git_str).into();
        let tar_mode: Mode = (&tar_str).into();
        let invalid_mode: Mode = (&invalid_str).into();

        assert_eq!(git_mode, Mode::Git);
        assert_eq!(tar_mode, Mode::Tar);
        assert_eq!(invalid_mode, Mode::Unknown);
    }
}