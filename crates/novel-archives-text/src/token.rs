use crate::term::Term;
use std::fmt::Write;
#[derive(Debug, PartialEq, Clone, new)]
pub struct TokenText(Vec<TokenKind>);

impl ToString for TokenText {
    fn to_string(&self) -> std::string::String {
        let mut s = String::new();
        for t in self.0.iter() {
            write!(&mut s, "{}", t.to_string()).unwrap();
        }
        s
    }
}

#[derive(Debug, PartialEq, Clone, new)]
pub enum TokenKind {
    Term {
        body: Span,
        term: Term,
    },
    Ruby {
        body: TokenText,
        ruby: TokenText,
        point: Position,
    },
    Spase(Span),
    Kanji(Span),
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
    ZenkakuNumber(Span),
    LinkAnnotation(Span),
    Annotation {
        marker: Span,
        description: TokenText,
    },
    Ignore(Span),
    Punctuation(Span),
    Other(Span),
    NewLine(Span),
}

impl ToString for TokenKind {
    fn to_string(&self) -> std::string::String {
        match self {
            TokenKind::Term { body, .. } => body.body().clone(),
            TokenKind::Ruby { body, .. } => body.to_string(),
            TokenKind::Spase(token) => token.body().clone(),
            TokenKind::Kanji(token) => token.body().clone(),
            TokenKind::Hiragana(token) => token.body().clone(),
            TokenKind::Katakana(token) => token.body().clone(),
            TokenKind::HalfKatakana(token) => token.body().clone(),
            TokenKind::Alphabet(token) => token.body().clone(),
            TokenKind::WideAlphabet(token) => token.body().clone(),
            TokenKind::ZenkakuAlphabet(token) => token.body().clone(),
            TokenKind::Digit { body, .. } => body.body().clone(),
            TokenKind::ZenkakuNumber(token) => token.body().clone(),
            TokenKind::LinkAnnotation(token) => token.body().clone(),
            TokenKind::Annotation { marker, .. } => marker.body().clone(),
            TokenKind::Ignore(token) => token.body().clone(),
            TokenKind::Punctuation(token) => token.body().clone(),
            TokenKind::Other(token) => token.body().clone(),
            TokenKind::NewLine(token) => token.body().to_string(),
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
