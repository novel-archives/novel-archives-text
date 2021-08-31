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
    Number(Part),
    Other(Part),
    NewLine,
}

#[derive(new, Getters)]
pub struct Part {
    body: String,
    point: Point,
}

#[derive(new, Getters)]
pub struct Point {
    line: usize,
    col: usize,
}
