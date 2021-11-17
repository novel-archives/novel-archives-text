use super::*;
pub type Error<'a> = nom::Err<nom::error::Error<token::ParsedSpan<'a>>>;

pub fn new_error(input: token::ParsedSpan, errkind: nom::error::ErrorKind) -> Error {
    nom::Err::Error(nom::error::Error::new(input, errkind))
}
