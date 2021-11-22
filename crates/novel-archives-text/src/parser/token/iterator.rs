use super::*;

#[derive(new, Debug, PartialEq)]
pub struct TextIterator<'a> {
    input: ParsedSpan<'a>,
    context: ParseContext,
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (input, parsed) = self.context.token(self.input).ok()?;
        self.input = input;
        Some(parsed)
    }
}

impl<'a> From<TextIterator<'a>> for TokenText {
    fn from(iter: TextIterator<'a>) -> Self {
        TokenText::new(iter.map(|parsed_token| parsed_token.into()).collect())
    }
}

#[cfg(test)]
mod tests {}
