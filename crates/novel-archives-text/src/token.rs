use crate::term::Term;
use crate::Id;
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
    Term { body: Span, term_id: Id<Term> },
    Ruby { body: Span, ruby: Span },
    KanjiRuby { body: Span, ruby: Span },
    Annotation { body: Span, description: TokenText },
    EmphasisMark(Span),
    Spase(Span),
    Ignore(Span),
    Plaintext(Span),
    NewLine(Span),
}

impl ToString for Token {
    fn to_string(&self) -> std::string::String {
        match self {
            Token::Term { body, .. } => format!("\"{}\"", body.body()),
            Token::Ruby { body, ruby } => format!("|{}《{}》", body.body(), ruby.body()),
            Token::EmphasisMark(body) => body.body().clone(),
            Token::Spase(body) => body.body().clone(),
            Token::KanjiRuby { body, ruby } => format!("{}({})", body.body(), ruby.body()),
            Token::Annotation { body, description } => {
                format!("|{}${}$", body.body(), description.to_string())
            }
            Token::Ignore(body) => body.body().clone(),
            Token::Plaintext(body) => body.body().clone(),
            Token::NewLine(body) => body.body().to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct Span {
    body: String,
    originel_position: Position,
}

#[derive(Debug, PartialEq, Clone, Default, new, Getters)]
pub struct Position {
    line: usize,
    byte_offset: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Token::new_kanji_ruby(Span::new("漢字".into(),Position::default()),Span::new("かんじ".into(),Position::default()))=>"漢字(かんじ)")]
    #[test_case(Token::new_spase(Span::new("  ".into(),Position::default()))=>"  ")]
    #[test_case(Token::new_ignore(Span::new("|".into(),Position::default()))=>"|")]
    #[test_case(Token::new_new_line(Span::new("\n".into(),Position::default()))=>"\n")]
    #[test_case(Token::new_term(Span::new("ほげ".into(),Position::default()),Id::new("term_id1"))=>"\"ほげ\"")]
    fn token_to_string_works(token: Token) -> String {
        token.to_string()
    }
}
