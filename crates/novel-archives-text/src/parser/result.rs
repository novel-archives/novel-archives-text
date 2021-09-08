use super::*;

pub type IResult<'a, T = token::Token<'a>> = Result<(token::Span<'a>, T), error::Error<'a>>;
