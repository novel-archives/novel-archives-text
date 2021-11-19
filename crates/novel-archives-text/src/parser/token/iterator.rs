use super::*;
use nom::branch::alt;

#[derive(new, Debug, PartialEq)]
pub struct RubyIterator<'a> {
    ruby: ParsedSpan<'a>,
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
}

impl<'a> Iterator for RubyBodyIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (body, parsed) = alt((
            complete::kanji,
            complete::space,
            complete::hiragana,
            complete::katakana,
            complete::half_and_wide_disit,
            complete::alphabet,
            complete::wide_alphabet,
            complete::half_katakana,
            complete::punctuation,
            complete::other_in_ruby_body,
        ))(self.body)
        .ok()?;
        self.body = body;
        Some(parsed)
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
}

impl<'a> Iterator for AnnotationBodyIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (body, parsed) = alt((
            complete::kanji_ruby,
            complete::space,
            complete::hiragana,
            complete::katakana,
            complete::half_and_wide_disit,
            complete::alphabet,
            complete::wide_alphabet,
            complete::half_katakana,
            complete::punctuation,
            complete::other_in_annotation_body,
        ))(self.body)
        .ok()?;
        self.body = body;
        Some(parsed)
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
        let (description, parsed) = alt((
            |input| self.context.term(input),
            complete::kanji_ruby,
            complete::space,
            complete::hiragana,
            complete::katakana,
            complete::half_and_wide_disit,
            complete::alphabet,
            complete::wide_alphabet,
            complete::half_katakana,
            complete::punctuation,
            complete::other_in_annotation_body,
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(""=>TokenText::new(vec![]);"empty")]
    #[test_case("感じ"=>TokenText::new(vec![
            Token::new_kanji(Span::new("感".into(),Position::new(1,0))),
            Token::new_hiragana(Span::new("じ".into(),Position::new(1,3))),
    ]))]
    #[test_case("感じ aｋ3３"=>TokenText::new(vec![
            Token::new_kanji(Span::new("感".into(),Position::new(1,0))),
            Token::new_hiragana(Span::new("じ".into(),Position::new(1,3))),
            Token::new_spase(Span::new(" ".into(),Position::new(1,6))),
            Token::new_alphabet(Span::new("a".into(),Position::new(1,7))),
            Token::new_wide_alphabet(Span::new("ｋ".into(),Position::new(1,8))),
            Token::new_digit(Span::new("3３".into(),Position::new(1,11)),33),
    ]))]
    #[test_case("缶じ *aｋ3３"=>TokenText::new(vec![
            Token::new_kanji(Span::new("缶".into(),Position::new(1,0))),
            Token::new_hiragana(Span::new("じ".into(),Position::new(1,3))),
            Token::new_spase(Span::new(" ".into(),Position::new(1,6))),
            Token::new_other(Span::new("*".into(),Position::new(1,7))),
            Token::new_alphabet(Span::new("a".into(),Position::new(1,8))),
            Token::new_wide_alphabet(Span::new("ｋ".into(),Position::new(1,9))),
            Token::new_digit(Span::new("3３".into(),Position::new(1,12)),33),
    ]))]
    fn ruby_iterator_works(input: &str) -> TokenText {
        let ruby_iter = RubyIterator::new(token::ParsedSpan::new(input));
        ruby_iter.into()
    }
}
