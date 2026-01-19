use std::fmt::Display;

/// The error type for Malva.
#[derive(Clone, Debug)]
pub enum Error {
    /// Error from the parser, usually related to syntax error.
    /// The first component is the error type from Raffia,
    /// and the second component is error line number,
    /// and the third component is error column number.
    Parser(raffia::error::Error, usize, usize),

    /// The specified range is outside of the source file bounds.
    RangeOutOfBounds {
        range: std::ops::Range<usize>,
        source_len: usize,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parser(error, line, col) => {
                write!(f, "syntax error at line {line}, col {col}: {}", error.kind)
            }
            Error::RangeOutOfBounds { range, source_len } => {
                write!(
                    f,
                    "range {}..{} is out of bounds (source length: {})",
                    range.start, range.end, source_len
                )
            }
        }
    }
}

impl std::error::Error for Error {}
