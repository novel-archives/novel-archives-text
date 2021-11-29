use super::*;
use std::sync::Arc;
pub mod complete;
mod context;
pub mod iterator;
mod span;

#[derive(Debug, PartialEq, Clone, new)]
pub enum ParsedToken<'a> {
    Term {
        body: ParsedSpan<'a>,
        term_id: Id<term::Term>,
    },
    Ruby {
        body: ParsedSpan<'a>,
        ruby: ParsedSpan<'a>,
    },
    KanjiRuby {
        body: ParsedSpan<'a>,
        ruby: ParsedSpan<'a>,
    },
    Annotation {
        body: ParsedSpan<'a>,
        description: iterator::TextIterator<'a>,
    },
    Space(ParsedSpan<'a>),
    EmphasisMark(ParsedSpan<'a>),
    Ignore(ParsedSpan<'a>),
    Plaintext(ParsedSpan<'a>),
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

impl<'a> From<ParsedToken<'a>> for crate::Token {
    fn from(token: ParsedToken<'a>) -> Self {
        match token {
            ParsedToken::Term { body, term_id } => Token::new_term(body.into(), term_id),
            ParsedToken::Ruby { body, ruby } => Token::new_ruby(body.into(), ruby.into()),
            ParsedToken::KanjiRuby { body, ruby } => {
                Token::new_kanji_ruby(body.into(), ruby.into())
            }
            ParsedToken::EmphasisMark(body) => Token::new_emphasis_mark(body.into()),
            ParsedToken::Space(body) => Token::new_spase(body.into()),
            ParsedToken::Annotation { body, description } => {
                Token::new_annotation(body.into(), description.collect())
            }
            ParsedToken::Ignore(body) => Token::new_ignore(body.into()),
            ParsedToken::Plaintext(body) => Token::new_plaintext(body.into()),
            ParsedToken::NewLine(body) => Token::NewLine(body.into()),
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
