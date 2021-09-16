use super::*;

pub type IResult<'a, T = token::Token<'a>> = nom::IResult<token::Span<'a>, T>;
