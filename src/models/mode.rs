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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "git" => Some(Mode::Git),
            "tar" => Some(Mode::Tar),
            _ => None,
        }
    }
}
