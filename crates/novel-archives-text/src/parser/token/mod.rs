use super::*;
pub mod complete;
pub mod iterator;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Term(Span<'a>),
    Ruby {
        body: iterator::RubyBodyIterator<'a>,
        ruby: iterator::RubyIterator<'a>,
    },
    KanjiRuby {
        kanji: Span<'a>,
        ruby: iterator::RubyIterator<'a>,
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
    NewLine(Span<'a>),
}

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[cfg(test)]
pub mod test_helper {
    use super::*;
    pub fn new_test_result_span(offset: usize, line: u32, fragment: &str) -> Span {
        unsafe { Span::new_from_raw_offset(offset, line, fragment, ()) }
    }
}
