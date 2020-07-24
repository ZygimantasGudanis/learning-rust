#![allow(dead_code)]

#[derive(Debug)]
pub enum Error {
    Custom(String),
    OutOfRange(String),
    NotImplemented(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "{}", s),
            Self::NotImplemented(s) => write!(f, "{}", s),
            Self::OutOfRange(s) => write!(f, "{}", s),
            //_ => write!(f, "This error has not been implementet yet"),
        }
    }
}

impl std::error::Error for Error {}
