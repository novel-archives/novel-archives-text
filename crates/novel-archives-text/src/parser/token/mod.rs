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
    LinkAnnotation {
        body: Vec<Token<'a>>,
        lined_at: usize,
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

#[cfg(test)]
pub mod test_helper {
    use super::*;
    pub fn new_test_result_span(offset: usize, line: u32, fragment: &str) -> Span {
        unsafe { Span::new_from_raw_offset(offset, line, fragment, ()) }
    }
}
