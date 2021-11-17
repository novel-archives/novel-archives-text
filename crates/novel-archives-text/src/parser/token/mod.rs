use super::*;
use std::sync::Arc;
pub mod complete;
mod context;
pub mod iterator;
mod span;

#[derive(Debug, PartialEq)]
pub enum ParsedToken<'a> {
    Term {
        body: ParsedSpan<'a>,
        term: Arc<term::Term>,
    },
    Ruby {
        body: iterator::RubyBodyIterator<'a>,
        ruby: iterator::RubyIterator<'a>,
    },
    KanjiRuby {
        body: ParsedSpan<'a>,
        ruby: iterator::RubyIterator<'a>,
    },
    Space(ParsedSpan<'a>),
    Kanji(ParsedSpan<'a>),
    Hiragana(ParsedSpan<'a>),
    Katakana(ParsedSpan<'a>),
    HalfKatakana(ParsedSpan<'a>),
    Alphabet(ParsedSpan<'a>),
    WideAlphabet(ParsedSpan<'a>),
    Digit {
        body: ParsedSpan<'a>,
        digit: usize,
    },
    Annotation {
        body: iterator::AnnotationBodyIterator<'a>,
        description: iterator::AnnotationDescriptionIterator<'a>,
    },
    Ignore(ParsedSpan<'a>),
    Punctuation(ParsedSpan<'a>),
    Other(ParsedSpan<'a>),
    NewLine(ParsedSpan<'a>),
}

pub use context::*;
pub use span::*;

use nom_extend::character;

fn without_variation_selector_count(input: &str) -> usize {
    input
        .chars()
        .filter(|&c| !character::is_kanji_variation_selector(c))
        .count()
}

impl<'a> From<ParsedToken<'a>> for crate::TokenKind {
    fn from(token: ParsedToken<'a>) -> Self {
        match token {
            ParsedToken::Term { body, term } => {
                TokenKind::new_term(body.into(), term.as_ref().clone())
            }
            ParsedToken::Ruby { .. } => todo!(),
            ParsedToken::KanjiRuby { .. } => todo!(),
            ParsedToken::Space(body) => TokenKind::new_spase(body.into()),
            ParsedToken::Kanji(body) => TokenKind::new_kanji(body.into()),
            ParsedToken::Hiragana(body) => TokenKind::new_hiragana(body.into()),
            ParsedToken::Katakana(body) => TokenKind::new_katakana(body.into()),
            ParsedToken::HalfKatakana(body) => TokenKind::new_half_katakana(body.into()),
            ParsedToken::Alphabet(body) => TokenKind::new_alphabet(body.into()),
            ParsedToken::WideAlphabet(body) => TokenKind::new_wide_alphabet(body.into()),
            ParsedToken::Digit { body, digit } => TokenKind::new_digit(body.into(), digit),
            ParsedToken::Annotation { .. } => todo!(),
            ParsedToken::Ignore(body) => TokenKind::new_ignore(body.into()),
            ParsedToken::Punctuation(body) => TokenKind::new_punctuation(body.into()),
            ParsedToken::Other(body) => TokenKind::new_other(body.into()),
            ParsedToken::NewLine(body) => TokenKind::NewLine(body.into()),
        }
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    pub fn new_test_result_span(offset: usize, line: u32, fragment: &str) -> ParsedSpan {
        unsafe { ParsedSpan::new_from_raw_offset(offset, line, fragment, ()) }
    }
}
