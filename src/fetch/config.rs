use crate::models::Mode;

#[derive(Debug)]
pub struct Config<'a> {
    pub dir: &'a String,
    pub mode: Mode,
    pub force: bool,
    pub keep_history: bool,
}

impl<'a> Config<'a> {
    pub fn from(dir: &'a String, mode: Mode, force: bool, keep_history: bool) -> Self {
        Self {
            dir,
            mode,
            force,
            keep_history,
        }
    }
}
