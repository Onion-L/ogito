use crate::models::Mode;

#[derive(Debug)]
pub struct Config<'a> {
    pub dir: &'a String,
    pub mode: Mode,
    pub cache: bool,
    pub force: bool,
    pub keep_history: bool,
    pub branch: Option<&'a String>,
}

impl<'a> Config<'a> {
    pub fn from(
        dir: &'a String,
        mode: Mode,
        cache: bool,
        force: bool,
        keep_history: bool,
        branch: Option<&'a String>,
    ) -> Self {
        Self {
            dir,
            mode,
            cache,
            force,
            keep_history,
            branch,
        }
    }
}
