use crate::models::{mode::Mode, site::Site};

#[derive(Debug)]
pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub site: Option<Site>,
    pub mode: Mode,
    pub force: bool,
}

impl<'a> Config<'a> {
    pub fn from(
        repo: Option<&'a String>,
        dir: Option<&'a String>,
        site: Option<Site>,
        mode: Mode,
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
