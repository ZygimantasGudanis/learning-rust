#![allow(dead_code)]

#[derive(Debug)]
pub enum Error {
    Custom(String),
    OutOfRange(String),
    NotImplemented(String),
    IntParse(std::num::ParseIntError),
    IO(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "{}", s),
            Self::NotImplemented(s) => write!(f, "{}", s),
            Self::OutOfRange(s) => write!(f, "{}", s),
            Self::IntParse(s) => write!(f, "{}", s),
            Self::IO(s) => write!(f, "{}", s),
            //_ => write!(f, "This error has not been implementet yet"),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::IntParse(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl std::error::Error for Error {}

