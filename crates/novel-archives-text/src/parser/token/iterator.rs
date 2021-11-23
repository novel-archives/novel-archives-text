use super::*;
use nom::branch::alt;
use std::iter::FromIterator;

#[derive(new, Debug, PartialEq)]
pub struct TextIterator<'a> {
    input: ParsedSpan<'a>,
    context: ParseContext,
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (input, parsed) = alt((
            |input| self.context.term(input),
            |input| self.context.directive_annotation(input),
            complete::kanji_ruby,
            complete::directive_ruby,
            complete::directive_other,
            complete::term_directive_other,
            complete::space,
            complete::newline,
        ))(self.input)
        .ok()?;
        self.input = input;
        Some(parsed)
    }
}

impl<'a> FromIterator<ParsedToken<'a>> for TokenText {
    fn from_iter<I: IntoIterator<Item = ParsedToken<'a>>>(iter: I) -> Self {
        struct PlainSpanHolder {
            body: String,
            position: Position,
        }
        let mut plain_span = None;
        let mut tokens = vec![];
        for parsed_token in iter.into_iter() {
            match parsed_token {
                ParsedToken::Plaintext(body) => {
                    if plain_span.is_none() {
                        plain_span = Some(PlainSpanHolder {
                            body: String::new(),
                            position: Position::new(
                                body.location_line() as usize,
                                body.location_offset(),
                            ),
                        })
                    }
                    let mut old_plain_span = plain_span.unwrap();
                    old_plain_span.body.push_str(body.fragment());
                    plain_span = Some(old_plain_span);
                }
                _ => {
                    if let Some(span) = plain_span {
                        tokens.push(Token::Plaintext(Span::new(span.body, span.position)));
                        plain_span = None;
                    }
                    tokens.push(parsed_token.into());
                }
            }
        }
        TokenText::new(tokens)
    }
}

#[cfg(test)]
mod tests {}
