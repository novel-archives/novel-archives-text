use crate::Position;

pub type ParsedSpan<'a> = nom_locate::LocatedSpan<&'a str>;

impl<'a> From<ParsedSpan<'a>> for crate::token::Span {
    fn from(span: ParsedSpan<'a>) -> Self {
        crate::Span::new(
            span.fragment().to_string(),
            Position::new(span.location_line() as usize, span.location_offset()),
        )
    }
}
