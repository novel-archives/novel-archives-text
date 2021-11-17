use crate::term::Term;
use std::fmt::Write;
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

#[derive(Debug, PartialEq, Clone, new)]
pub enum Token {
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

impl ToString for Token {
    fn to_string(&self) -> std::string::String {
        match self {
            Token::Term { body, .. } => body.body().clone(),
            Token::Ruby { body, .. } => body.to_string(),
            Token::Spase(token) => token.body().clone(),
            Token::Kanji(token) => token.body().clone(),
            Token::Hiragana(token) => token.body().clone(),
            Token::Katakana(token) => token.body().clone(),
            Token::HalfKatakana(token) => token.body().clone(),
            Token::Alphabet(token) => token.body().clone(),
            Token::WideAlphabet(token) => token.body().clone(),
            Token::ZenkakuAlphabet(token) => token.body().clone(),
            Token::Digit { body, .. } => body.body().clone(),
            Token::ZenkakuNumber(token) => token.body().clone(),
            Token::LinkAnnotation(token) => token.body().clone(),
            Token::Annotation { marker, .. } => marker.body().clone(),
            Token::Ignore(token) => token.body().clone(),
            Token::Punctuation(token) => token.body().clone(),
            Token::Other(token) => token.body().clone(),
            Token::NewLine(token) => token.body().to_string(),
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
