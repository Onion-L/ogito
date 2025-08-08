use crate::models::Mode;

#[derive(Debug)]
pub struct Config<'a> {
    pub dir: &'a String,
    pub mode: Mode,
    pub force: bool,
    pub keep_history: bool,
    pub branch: Option<&'a String>,
}

impl<'a> Config<'a> {
    pub fn from(
        dir: &'a String,
        mode: Mode,
        force: bool,
        keep_history: bool,
        branch: Option<&'a String>,
    ) -> Self {
        Self {
            dir,
            mode,
            force,
            keep_history,
            branch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::models::Mode;

    #[test]
    fn test_config_from_all_fields() {
        let dir = String::from("dir");
        let mode = Mode::Git;
        let branch = String::from("branch");
        let config = Config::from(&dir, mode.clone(), false, false, Some(&branch));
        assert_eq!(config.dir, &dir);
        assert_eq!(config.mode, mode);
        assert!(!config.force);
        assert!(!config.keep_history);
    }
}
