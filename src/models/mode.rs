#[derive(Debug, Clone)]
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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "git" => Ok(Mode::Git),
            "tar" => Ok(Mode::Tar),
            _ => Err(format!("Invalid mode: {}", s)),
        }
    }
}
