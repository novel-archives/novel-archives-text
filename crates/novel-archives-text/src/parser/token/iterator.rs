use super::*;
use nom::branch::alt;

#[derive(new, Debug, PartialEq)]
pub struct AnnotationDescriptionIterator<'a> {
    description: ParsedSpan<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationDescriptionIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (description, parsed) = alt((
            |input| self.context.term(input),
            complete::kanji_ruby,
            complete::directive_ruby,
            complete::directive_other,
            complete::space,
            complete::plaintext,
        ))(self.description)
        .ok()?;
        self.description = description;
        Some(parsed)
    }
}

impl<'a> From<AnnotationDescriptionIterator<'a>> for TokenText {
    fn from(iter: AnnotationDescriptionIterator<'a>) -> Self {
        Self::new(iter.map(|pt| pt.into()).collect())
    }
}

#[derive(new, Debug, PartialEq)]
pub struct TextIterator<'a> {
    input: ParsedSpan<'a>,
    context: Context,
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
            complete::space,
            complete::newline,
            complete::plaintext,
        ))(self.input)
        .ok()?;
        self.input = input;
        Some(parsed)
    }
}

#[cfg(test)]
mod tests {}
