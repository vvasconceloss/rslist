use std::fmt;

#[derive(Debug)]
pub enum Errors {
    IO(std::io::Error),
}

impl From<std::io::Error> for Errors {
    fn from(err: std::io::Error) -> Self {
        Errors::IO(err)
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::IO(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for Errors {}
