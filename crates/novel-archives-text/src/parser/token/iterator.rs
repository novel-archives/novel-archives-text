use super::*;
use nom::branch::alt;

#[derive(new, Debug, PartialEq)]
pub struct RubyIterator<'a> {
    ruby: Span<'a>,
    context: Context,
}

impl<'a> Iterator for RubyIterator<'a> {
    type Item = Token<'a>;
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

#[derive(new, Debug, PartialEq)]
pub struct RubyBodyIterator<'a> {
    body: Span<'a>,
    context: Context,
}

impl<'a> Iterator for RubyBodyIterator<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[derive(new, Debug, PartialEq)]
pub struct AnnotationBodyIterator<'a> {
    body: Span<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationBodyIterator<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[derive(new, Debug, PartialEq)]
pub struct AnnotationDescriptionIterator<'a> {
    description: Span<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationDescriptionIterator<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}
