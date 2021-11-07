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
    Term(Token),
    Ruby {
        target: TokenText,
        ruby: TokenText,
        point: Position,
    },
    Spase(Token),
    Kanji(Token),
    Hiragana(Token),
    Katakana(Token),
    Alphabet(Token),
    ZenkakuAlphabet(Token),
    Number(Token),
    ZenkakuNumber(Token),
    LinkAnnotation(Token),
    Annotation {
        marker: Token,
        description: TokenText,
    },
    Other(Token),
    NewLine,
}

impl ToString for TokenKind {
    fn to_string(&self) -> std::string::String {
        match self {
            TokenKind::Term(token) => token.body().clone(),
            TokenKind::Ruby { target, .. } => target.to_string(),
            TokenKind::Spase(token) => token.body().clone(),
            TokenKind::Kanji(token) => token.body().clone(),
            TokenKind::Hiragana(token) => token.body().clone(),
            TokenKind::Katakana(token) => token.body().clone(),
            TokenKind::Alphabet(token) => token.body().clone(),
            TokenKind::ZenkakuAlphabet(token) => token.body().clone(),
            TokenKind::Number(token) => token.body().clone(),
            TokenKind::ZenkakuNumber(token) => token.body().clone(),
            TokenKind::LinkAnnotation(token) => token.body().clone(),
            TokenKind::Annotation { marker, .. } => marker.body().clone(),
            TokenKind::Other(token) => token.body().clone(),
            TokenKind::NewLine => "\n".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct Token {
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
