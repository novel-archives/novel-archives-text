pub enum TokenKind {
    Term(Token),
    Ruby {
        parts: Vec<TokenKind>,
        ruby: Vec<TokenKind>,
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
        desription: Vec<TokenKind>,
    },
    Other(Token),
    NewLine,
}

#[derive(new, Getters)]
pub struct Token {
    body: String,
    originel_position: Position,
}

#[derive(new, Getters)]
pub struct Position {
    line: usize,
    byte_offset: usize,
}
