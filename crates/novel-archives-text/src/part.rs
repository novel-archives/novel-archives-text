pub enum PartKind {
    Term(Part),
    WithRuby {
        parts: Vec<PartKind>,
        ruby: Vec<PartKind>,
        point: Point,
    },
    Spase(Part),
    Kanji(Part),
    Hiragana(Part),
    Katakana(Part),
    Alphabet(Part),
    ZenkakuAlphabet(Part),
    Number(Part),
    ZenkakuNumber(Part),
    LinkAnnotation(Part),
    Annotation {
        marker: Part,
        desription: Vec<PartKind>,
    },
    Other(Part),
    NewLine,
}

#[derive(new, Getters)]
pub struct Part {
    body: String,
    range: Range,
}

#[derive(new, Getters)]
pub struct Range {
    start: Point,
    end: Point,
}
#[derive(new, Getters)]
pub struct Point {
    line: usize,
    col: usize,
}
