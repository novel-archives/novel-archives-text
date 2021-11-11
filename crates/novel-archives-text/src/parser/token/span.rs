use crate::Position;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;

impl<'a> From<Span<'a>> for crate::token::Token {
    fn from(span: Span<'a>) -> Self {
        crate::Token::new(
            span.fragment().to_string(),
            Position::new(span.location_line() as usize, span.location_offset()),
        )
    }
}
