use crate::term::Term;
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
    Term {
        body: Span,
        term: Term,
    },
    Ruby {
        body: TokenText,
        ruby: TokenText,
    },
    Spase(Span),
    Kanji(Span),
    KanjiRuby {
        body: Span,
        ruby: TokenText,
    },
    Hiragana(Span),
    Katakana(Span),
    HalfKatakana(Span),
    Alphabet(Span),
    WideAlphabet(Span),
    Digit {
        body: Span,
        digit: usize,
    },
    Annotation {
        marker: TokenText,
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
            Token::Ruby { body, ruby } => format!("|{}《{}》", body.to_string(), ruby.to_string()),
            Token::Spase(body) => body.body().clone(),
            Token::Kanji(body) => body.body().clone(),
            Token::KanjiRuby { body, ruby } => format!("{}({})", body.body(), ruby.to_string()),
            Token::Hiragana(body) => body.body().clone(),
            Token::Katakana(body) => body.body().clone(),
            Token::HalfKatakana(body) => body.body().clone(),
            Token::Alphabet(body) => body.body().clone(),
            Token::WideAlphabet(body) => body.body().clone(),
            Token::Digit { body, .. } => body.body().clone(),
            Token::Annotation {
                marker,
                description,
            } => format!("|{}${}$", marker.to_string(), description.to_string()),
            Token::Ignore(body) => body.body().clone(),
            Token::Punctuation(body) => body.body().clone(),
            Token::Other(body) => body.body().clone(),
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

    #[test_case(Token::new_kanji(Span::new("漢字".into(),Position::default()))=>"漢字")]
    #[test_case(Token::new_kanji_ruby(Span::new("漢字".into(),Position::default()),TokenText::new(vec![Token::new_hiragana(Span::new("かんじ".into(),Position::default()))]))=>"漢字(かんじ)")]
    #[test_case(Token::new_ruby(TokenText::new(vec![Token::new_hiragana(Span::new("暗黒".into(),Position::default()))]),TokenText::new(vec![Token::new_hiragana(Span::new("ダークパワー".into(),Position::default()))]))=>"|暗黒《ダークパワー》")]
    #[test_case(Token::new_annotation(TokenText::new(vec![Token::new_hiragana(Span::new("暗黒".into(),Position::default()))]),TokenText::new(vec![Token::new_hiragana(Span::new("光と闇の力が合わさり最強に見える".into(),Position::default()))]))=>"|暗黒$光と闇の力が合わさり最強に見える$")]
    #[test_case(Token::new_spase(Span::new("  ".into(),Position::default()))=>"  ")]
    #[test_case(Token::new_hiragana(Span::new("あいう".into(),Position::default()))=>"あいう")]
    #[test_case(Token::new_katakana(Span::new("アイウ".into(),Position::default()))=>"アイウ")]
    #[test_case(Token::new_half_katakana(Span::new("ｱｲｳ".into(),Position::default()))=>"ｱｲｳ")]
    #[test_case(Token::new_alphabet(Span::new("abc".into(),Position::default()))=>"abc")]
    #[test_case(Token::new_wide_alphabet(Span::new("ｂｃｄ".into(),Position::default()))=>"ｂｃｄ")]
    #[test_case(Token::new_digit(Span::new("3３".into(),Position::default()),33)=>"3３")]
    #[test_case(Token::new_ignore(Span::new("|".into(),Position::default()))=>"|")]
    #[test_case(Token::new_punctuation(Span::new("。".into(),Position::default()))=>"。")]
    #[test_case(Token::new_other(Span::new("#".into(),Position::default()))=>"#")]
    #[test_case(Token::new_new_line(Span::new("\n".into(),Position::default()))=>"\n")]
    fn token_to_string_works(token: Token) -> String {
        token.to_string()
    }
}
