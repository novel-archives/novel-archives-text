pub enum Token<'a> {
    Term(Span<'a>),
    Ruby {
        parts: Vec<Token<'a>>,
        ruby: Vec<Token<'a>>,
        point: crate::Position,
    },
    Spase(Span<'a>),
    Kanji(Span<'a>),
    Hiragana(Span<'a>),
    Katakana(Span<'a>),
    Alphabet(Span<'a>),
    ZenkakuAlphabet(Span<'a>),
    Number(Span<'a>),
    ZenkakuNumber(Span<'a>),
    LinkAnnotation(Span<'a>),
    Annotation {
        marker: Span<'a>,
        desription: Vec<Token<'a>>,
    },
    Other(Span<'a>),
    NewLine,
}

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
