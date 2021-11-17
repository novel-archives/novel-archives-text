use super::*;
use nom::branch::alt;

#[derive(new, Debug, PartialEq)]
pub struct RubyIterator<'a> {
    ruby: ParsedSpan<'a>,
    context: Context,
}

impl<'a> Iterator for RubyIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (ruby, parsed) = alt((
            complete::kanji,
            complete::space,
            complete::hiragana,
            complete::katakana,
            complete::half_and_wide_disit,
            complete::alphabet,
            complete::wide_alphabet,
            complete::half_katakana,
            complete::punctuation,
            complete::other_in_ruby,
        ))(self.ruby)
        .ok()?;
        self.ruby = ruby;
        Some(parsed)
    }
}

impl<'a> From<RubyIterator<'a>> for TokenText {
    fn from(iter: RubyIterator<'a>) -> Self {
        Self::new(iter.map(|pt| pt.into()).collect())
    }
}

#[derive(new, Debug, PartialEq)]
pub struct RubyBodyIterator<'a> {
    body: ParsedSpan<'a>,
    context: Context,
}

impl<'a> Iterator for RubyBodyIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

impl<'a> From<RubyBodyIterator<'a>> for TokenText {
    fn from(iter: RubyBodyIterator<'a>) -> Self {
        Self::new(iter.map(|pt| pt.into()).collect())
    }
}

#[derive(new, Debug, PartialEq)]
pub struct AnnotationBodyIterator<'a> {
    body: ParsedSpan<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationBodyIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

impl<'a> From<AnnotationBodyIterator<'a>> for TokenText {
    fn from(iter: AnnotationBodyIterator<'a>) -> Self {
        Self::new(iter.map(|pt| pt.into()).collect())
    }
}

#[derive(new, Debug, PartialEq)]
pub struct AnnotationDescriptionIterator<'a> {
    description: ParsedSpan<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationDescriptionIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

impl<'a> From<AnnotationDescriptionIterator<'a>> for TokenText {
    fn from(iter: AnnotationDescriptionIterator<'a>) -> Self {
        Self::new(iter.map(|pt| pt.into()).collect())
    }
}
