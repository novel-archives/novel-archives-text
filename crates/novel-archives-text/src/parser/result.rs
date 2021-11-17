use super::*;

pub type IResult<'a, T = token::ParsedToken<'a>> = nom::IResult<token::Span<'a>, T>;
