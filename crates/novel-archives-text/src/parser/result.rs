use super::*;

pub type IResult<'a> = Result<(token::Span<'a>, token::Token<'a>), error::Error<'a>>;
