use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Error {
    Parser(raffia::error::Error),
    Fmt(std::fmt::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parser(error) => write!(f, "syntax error: {}", error.kind),
            Error::Fmt(error) => write!(f, "failed to format as string: {error}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<raffia::error::Error> for Error {
    fn from(error: raffia::error::Error) -> Self {
        Error::Parser(error)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
