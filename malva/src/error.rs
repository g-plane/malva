#[derive(Clone, Debug)]
pub enum Error {
    Parser(raffia::error::Error),
    Fmt(std::fmt::Error),
}

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
