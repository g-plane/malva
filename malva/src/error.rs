use std::fmt::Display;

/// The error type for Malva.
#[derive(Clone, Debug)]
pub enum Error {
    /// Error from the parser, usually related to syntax error.
    /// The first component is the error type from Raffia,
    /// and the second component is error line number,
    /// and the third component is error column number.
    Parser(raffia::error::Error, usize, usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parser(error, line, col) => {
                write!(f, "syntax error at line {line}, col {col}: {}", error.kind)
            }
        }
    }
}

impl std::error::Error for Error {}
