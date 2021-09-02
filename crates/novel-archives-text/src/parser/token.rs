pub enum TokenKind<'a> {
    Term(Token<'a>),
    Ruby {
        parts: Vec<TokenKind<'a>>,
        ruby: Vec<TokenKind<'a>>,
        point: crate::Position,
    },
    Spase(Token<'a>),
    Kanji(Token<'a>),
    Hiragana(Token<'a>),
    Katakana(Token<'a>),
    Alphabet(Token<'a>),
    ZenkakuAlphabet(Token<'a>),
    Number(Token<'a>),
    ZenkakuNumber(Token<'a>),
    LinkAnnotation(Token<'a>),
    Annotation {
        marker: Token<'a>,
        desription: Vec<TokenKind<'a>>,
    },
    Other(Token<'a>),
    NewLine,
}

#[derive(new, Getters)]
pub struct Token<'a> {
    body: &'a str,
    originel_position: crate::Position,
}
