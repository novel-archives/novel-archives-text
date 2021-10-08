use super::*;

pub type IResult<'a, 'b, T = token::Token<'a, 'b>> = nom::IResult<token::Span<'a>, T>;
