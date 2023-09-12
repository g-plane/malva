use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Error {
    Parser(raffia::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parser(error) => write!(f, "syntax error: {}", error.kind),
        }
    }
}

impl std::error::Error for Error {}

impl From<raffia::error::Error> for Error {
    fn from(error: raffia::error::Error) -> Self {
        Error::Parser(error)
    }
}
