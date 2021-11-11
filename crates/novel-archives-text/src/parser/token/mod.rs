use super::*;
use std::sync::Arc;
pub mod complete;
mod context;
pub mod iterator;
mod span;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Term {
        body: Span<'a>,
        term: Arc<term::Term>,
    },
    Ruby {
        body: iterator::RubyBodyIterator<'a>,
        ruby: iterator::RubyIterator<'a>,
    },
    KanjiRuby {
        body: Span<'a>,
        ruby: iterator::RubyIterator<'a>,
    },
    Space(Span<'a>),
    Kanji(Span<'a>),
    Hiragana(Span<'a>),
    Katakana(Span<'a>),
    HalfKatakana(Span<'a>),
    Alphabet(Span<'a>),
    WideAlphabet(Span<'a>),
    Digit {
        body: Span<'a>,
        digit: usize,
    },
    Annotation {
        body: iterator::AnnotationBodyIterator<'a>,
        description: iterator::AnnotationDescriptionIterator<'a>,
    },
    Ignore(Span<'a>),
    Punctuation(Span<'a>),
    Other(Span<'a>),
    NewLine(Span<'a>),
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

impl<'a> From<Token<'a>> for crate::TokenKind {
    fn from(token: Token<'a>) -> Self {
        match token {
            Token::Term { body, term } => TokenKind::new_term(body.into(), term.as_ref().clone()),
            Token::Ruby { .. } => todo!(),
            Token::KanjiRuby { .. } => todo!(),
            Token::Space(body) => TokenKind::new_spase(body.into()),
            Token::Kanji(body) => TokenKind::new_kanji(body.into()),
            Token::Hiragana(body) => TokenKind::new_hiragana(body.into()),
            Token::Katakana(body) => TokenKind::new_katakana(body.into()),
            Token::HalfKatakana(body) => TokenKind::new_half_katakana(body.into()),
            Token::Alphabet(body) => TokenKind::new_alphabet(body.into()),
            Token::WideAlphabet(body) => TokenKind::new_wide_alphabet(body.into()),
            Token::Digit { body, digit } => TokenKind::new_digit(body.into(), digit),
            Token::Annotation { .. } => todo!(),
            Token::Ignore(body) => TokenKind::new_ignore(body.into()),
            Token::Punctuation(body) => TokenKind::new_punctuation(body.into()),
            Token::Other(body) => TokenKind::new_other(body.into()),
            Token::NewLine(body) => TokenKind::NewLine(body.into()),
        }
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    pub fn new_test_result_span(offset: usize, line: u32, fragment: &str) -> Span {
        unsafe { Span::new_from_raw_offset(offset, line, fragment, ()) }
    }
}
