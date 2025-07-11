use crate::models::{mode::Mode, site::Site};

#[derive(Debug, Default)]
pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub site: Option<Site>,
    pub mode: Option<&'a Mode>,
    pub force: bool,
}

impl<'a> Config<'a> {
    pub fn from(
        repo: Option<&'a String>,
        dir: Option<&'a String>,
        site: Option<Site>,
        mode: Option<&'a Mode>,
        force: bool,
    ) -> Self {
        Self {
            repo,
            dir,
            site,
            mode,
            force,
        }
    }
}
