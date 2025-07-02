use std::ffi::OsString;

pub struct App {
    pub directories: Vec<OsString>,
    pub files: Vec<OsString>,
}

impl App {
    pub fn new() -> Self {
        Self {
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn from(directories: Vec<OsString>, files: Vec<OsString>) -> Self {
        Self { directories, files }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_from() {
        let dirs: Vec<OsString> = Vec::new();
        let f: Vec<OsString> = Vec::new();
        let app = App::from(dirs.clone(), f.clone());
        assert_eq!(app.directories, dirs);
        assert_eq!(app.files, f);
    }
}
