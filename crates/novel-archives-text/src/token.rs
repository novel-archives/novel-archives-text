use crate::term::Term;
use std::{fmt::Write, ops::Deref};
#[derive(Debug, PartialEq, Clone, new)]
pub struct TokenText(Vec<Token>);

impl ToString for TokenText {
    fn to_string(&self) -> std::string::String {
        let mut s = String::new();
        for t in self.0.iter() {
            write!(&mut s, "{}", t.to_string()).unwrap();
        }
        s
    }
}

impl Deref for TokenText {
    type Target = Vec<Token>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone, new)]
pub enum Token {
    Term {
        body: Span,
        term: Term,
    },
    Ruby {
        body: TokenText,
        ruby: TokenText,
    },
    Spase(Span),
    Kanji(Span),
    KanjiRuby {
        body: Span,
        ruby: TokenText,
    },
    Hiragana(Span),
    Katakana(Span),
    HalfKatakana(Span),
    Alphabet(Span),
    WideAlphabet(Span),
    ZenkakuAlphabet(Span),
    Digit {
        body: Span,
        digit: usize,
    },
    Annotation {
        marker: TokenText,
        description: TokenText,
    },
    Ignore(Span),
    Punctuation(Span),
    Other(Span),
    NewLine(Span),
}

impl ToString for Token {
    fn to_string(&self) -> std::string::String {
        match self {
            Token::Term { body, .. } => body.body().clone(),
            Token::Ruby { body, ruby } => format!("|{}《{}》", body.to_string(), ruby.to_string()),
            Token::Spase(body) => body.body().clone(),
            Token::Kanji(body) => body.body().clone(),
            Token::KanjiRuby { body, ruby } => format!("{}({})", body.body(), ruby.to_string()),
            Token::Hiragana(body) => body.body().clone(),
            Token::Katakana(body) => body.body().clone(),
            Token::HalfKatakana(body) => body.body().clone(),
            Token::Alphabet(body) => body.body().clone(),
            Token::WideAlphabet(body) => body.body().clone(),
            Token::ZenkakuAlphabet(body) => body.body().clone(),
            Token::Digit { body, .. } => body.body().clone(),
            Token::Annotation {
                marker,
                description,
            } => format!("|{}${}$", marker.to_string(), description.to_string()),
            Token::Ignore(body) => body.body().clone(),
            Token::Punctuation(body) => body.body().clone(),
            Token::Other(body) => body.body().clone(),
            Token::NewLine(body) => body.body().to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct Span {
    body: String,
    originel_position: Position,
}

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct Position {
    line: usize,
    byte_offset: usize,
}

#[cfg(test)]
mod test_helper {}
