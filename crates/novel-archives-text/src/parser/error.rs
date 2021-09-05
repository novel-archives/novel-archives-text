use super::*;
use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum Error<'a> {
    #[error("incomplete")]
    Incomplete,
    #[error("nom_error:{0:?},{1:?}")]
    Nom(token::Span<'a>, nom::error::ErrorKind),
}

impl<'a> From<nom::Err<nom::error::Error<token::Span<'a>>>> for Error<'a> {
    fn from(err: nom::Err<nom::error::Error<token::Span<'a>>>) -> Self {
        match err {
            nom::Err::Error(err) | nom::Err::Failure(err) => Self::Nom(err.input, err.code),
            nom::Err::Incomplete(_) => Self::Incomplete,
        }
    }
}
