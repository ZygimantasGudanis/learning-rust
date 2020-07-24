#[derive(Debug)]
pub enum Error {
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}
