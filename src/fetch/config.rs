use crate::models::{mode::Mode, site::Site};

#[derive(Debug, Default)]
pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub site: Option<&'a Site>,
    pub mode: Option<&'a Mode>,
    pub force: bool,
}
