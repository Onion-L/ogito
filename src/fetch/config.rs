use crate::models::Mode;

#[derive(Debug)]
pub struct Config<'a> {
    pub repo: Option<&'a String>,
    pub dir: Option<&'a String>,
    pub mode: Mode,
    pub force: bool,
}

impl<'a> Config<'a> {
    pub fn from(
        repo: Option<&'a String>,
        dir: Option<&'a String>,
        mode: Mode,
        force: bool,
    ) -> Self {
        Self {
            repo,
            dir,
            mode,
            force,
        }
    }
}
