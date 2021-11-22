use super::*;
pub mod complete;
pub use nom::character::*;
pub fn is_any_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '　'
}

pub fn is_start_directive(c: char) -> bool {
    c == '|' || c == '｜'
}
pub fn is_start_annotation(c: char) -> bool {
    c == '$' || c == '＄'
}

pub fn is_end_annotation(c: char) -> bool {
    c == '$' || c == '＄'
}

pub fn is_start_ruby(c: char) -> bool {
    c == '《' || c == '⟪' || c == '(' || c == '（'
}

pub fn is_end_ruby(c: char) -> bool {
    c == '》' || c == '⟫' || c == ')' || c == '）'
}

pub fn is_kanji_related(c: char) -> bool {
    is_kanji(c) || is_kanji_extend(c) || is_kanji_variation_selector(c)
}

pub fn is_kanji(c: char) -> bool {
    kanji::is_kanji(&c)
}
pub fn is_kanji_extend(c: char) -> bool {
    kanji::is_kanji_extended(&c)
}

#[allow(clippy::manual_range_contains)]
pub fn is_kanji_variation_selector(c: char) -> bool {
    c >= '\u{e0100}' && c <= '\u{e01ef}'
}

pub fn is_hiragana(c: char) -> bool {
    kanji::is_hiragana(&c)
}

pub fn is_katakana(c: char) -> bool {
    kanji::is_katakana(&c)
}

#[allow(clippy::manual_range_contains)]
pub fn is_wide_alphabetic(c: char) -> bool {
    (c >= 'ａ' && c <= 'ｚ') || (c >= 'Ａ' && c <= 'Ｚ')
}

#[allow(clippy::manual_range_contains)]
pub fn is_wide_half_alphabetic(c: char) -> bool {
    is_wide_alphabetic(c) || ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'))
}

#[allow(clippy::manual_range_contains)]
pub fn is_wide_disit(c: char) -> bool {
    c >= '０' && c <= '９'
}

#[allow(clippy::manual_range_contains)]
pub fn is_half_disit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[allow(clippy::manual_range_contains)]
pub fn is_wide_half_disit(c: char) -> bool {
    is_wide_disit(c) || is_half_disit(c)
}

pub fn wide_half_disit_char_to_disit(c: char) -> Option<u32> {
    if is_half_disit(c) {
        Some(c as u32 - '0' as u32)
    } else if is_wide_disit(c) {
        Some(c as u32 - '０' as u32)
    } else {
        None
    }
}

pub fn is_start_link_annotation(c: char) -> bool {
    c == '*' || c == '＊'
}

pub fn is_able_to_ruby(c: char) -> bool {
    !(is_any_newline(c) || is_end_ruby(c))
}

pub fn is_able_to_ruby_body(c: char) -> bool {
    !(is_any_newline(c) || is_start_ruby(c))
}

pub fn is_any_newline(c: char) -> bool {
    c == '\r' || c == '\n'
}

#[allow(clippy::manual_range_contains)]
pub fn is_half_katakana(c: char) -> bool {
    c >= 'ｦ' && c <= 'ﾝ'
}

pub fn is_punctuation(c: char) -> bool {
    c == '、' || c == '。'
}

pub fn is_able_to_annotation_body(c: char) -> bool {
    !(is_any_newline(c) || is_start_annotation(c))
}

pub fn is_able_to_annotation(c: char) -> bool {
    !(is_any_newline(c) || is_end_annotation(c))
}

pub fn is_plaintext(c: char) -> bool {
    !(is_start_directive(c) || is_any_space(c) || is_any_newline(c) || is_kanji(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[allow(clippy::bool_assert_comparison)]
    #[test_case(' '=>true;"half_space")]
    #[test_case('　'=>true;"wide_space")]
    #[test_case('\t'=>true;"tab")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_any_space_works(c: char) -> bool {
        is_any_space(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('|'=>true;"half_pipe")]
    #[test_case('｜'=>true;"wide_pipe")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_start_directive_works(c: char) -> bool {
        is_start_directive(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('$'=>true;"half_$")]
    #[test_case('＄'=>true;"wide_＄")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_start_annotation_works(c: char) -> bool {
        is_start_annotation(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('$'=>true;"half_$")]
    #[test_case('＄'=>true;"wide_＄")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_end_annotation_works(c: char) -> bool {
        is_end_annotation(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('《'=>true;"normal")]
    #[test_case('⟪'=>true;"mathematical")]
    #[test_case('('=>true;"half")]
    #[test_case('（'=>true;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_start_ruby_works(c: char) -> bool {
        is_start_ruby(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('》'=>true;"normal")]
    #[test_case('⟫'=>true;"mathematical")]
    #[test_case(')'=>true;"half")]
    #[test_case('）'=>true;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_end_ruby_works(c: char) -> bool {
        is_end_ruby(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>true)]
    #[test_case('漢'=>true)]
    fn is_kanji_works(c: char) -> bool {
        is_kanji(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>true)]
    #[test_case('ア'=>false)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_hiragana_works(c: char) -> bool {
        is_hiragana(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('ア'=>true)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_katakana_works(c: char) -> bool {
        is_katakana(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('ａ'=>true)]
    #[test_case('ｂ'=>true)]
    #[test_case('ｃ'=>true)]
    #[test_case('ｚ'=>true)]
    #[test_case('Ａ'=>true)]
    #[test_case('Ｂ'=>true)]
    #[test_case('Ｃ'=>true)]
    #[test_case('Ｚ'=>true)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('ア'=>false)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_wide_alphabetic_works(c: char) -> bool {
        is_wide_alphabetic(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>true)]
    #[test_case('b'=>true)]
    #[test_case('z'=>true)]
    #[test_case('ａ'=>true)]
    #[test_case('ｂ'=>true)]
    #[test_case('ｃ'=>true)]
    #[test_case('ｚ'=>true)]
    #[test_case('Ａ'=>true)]
    #[test_case('Ｂ'=>true)]
    #[test_case('Ｃ'=>true)]
    #[test_case('Ｚ'=>true)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('ア'=>false)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_wide_half_alphabetic_works(c: char) -> bool {
        is_wide_half_alphabetic(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('ａ'=>false)]
    #[test_case('ｂ'=>false)]
    #[test_case('ｃ'=>false)]
    #[test_case('ｚ'=>false)]
    #[test_case('Ａ'=>false)]
    #[test_case('Ｂ'=>false)]
    #[test_case('Ｃ'=>false)]
    #[test_case('Ｚ'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>true)]
    #[test_case('１'=>true)]
    #[test_case('9'=>false)]
    #[test_case('８'=>true)]
    #[test_case('９'=>true)]
    #[test_case('あ'=>false)]
    #[test_case('ア'=>false)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_wide_disit_works(c: char) -> bool {
        is_wide_disit(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('ａ'=>false)]
    #[test_case('ｂ'=>false)]
    #[test_case('ｃ'=>false)]
    #[test_case('ｚ'=>false)]
    #[test_case('Ａ'=>false)]
    #[test_case('Ｂ'=>false)]
    #[test_case('Ｃ'=>false)]
    #[test_case('Ｚ'=>false)]
    #[test_case('0'=>true)]
    #[test_case('1'=>true)]
    #[test_case('０'=>true)]
    #[test_case('１'=>true)]
    #[test_case('8'=>true)]
    #[test_case('9'=>true)]
    #[test_case('８'=>true)]
    #[test_case('９'=>true)]
    #[test_case('あ'=>false)]
    #[test_case('ア'=>false)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_wide_half_disit_works(c: char) -> bool {
        is_wide_half_disit(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"half")]
    #[test_case('（'=>false;"wide")]
    #[test_case('*'=>true;"half_link_annotation")]
    #[test_case('＊'=>true;"wide_link_annotation")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('ａ'=>false)]
    #[test_case('ｂ'=>false)]
    #[test_case('ｃ'=>false)]
    #[test_case('ｚ'=>false)]
    #[test_case('Ａ'=>false)]
    #[test_case('Ｂ'=>false)]
    #[test_case('Ｃ'=>false)]
    #[test_case('Ｚ'=>false)]
    #[test_case('0'=>false)]
    #[test_case('1'=>false)]
    #[test_case('０'=>false)]
    #[test_case('１'=>false)]
    #[test_case('8'=>false)]
    #[test_case('9'=>false)]
    #[test_case('８'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('ア'=>false)]
    #[test_case('塡'=>false)]
    #[test_case('漢'=>false)]
    fn is_start_link_annotation_works(c: char) -> bool {
        is_start_link_annotation(c)
    }

    #[test_case('０'=>Some(0))]
    #[test_case('１'=>Some(1))]
    #[test_case('９'=>Some(9))]
    #[test_case('0'=>Some(0))]
    #[test_case('9'=>Some(9))]
    #[test_case('h'=>None)]
    fn wide_half_disit_to_disit_works(c: char) -> Option<u32> {
        wide_half_disit_char_to_disit(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('\n'=>true)]
    #[test_case('\r'=>true)]
    #[test_case('\t'=>false)]
    fn is_any_newline_works(c: char) -> bool {
        is_any_newline(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('\n'=>false)]
    #[test_case('\r'=>false)]
    #[test_case('\t'=>true)]
    fn is_able_to_ruby_works(c: char) -> bool {
        is_able_to_ruby(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('ア'=>false)]
    #[test_case('ｱ'=>true)]
    #[test_case('イ'=>false)]
    #[test_case('ｲ'=>true)]
    #[test_case('ヲ'=>false)]
    #[test_case('ｦ'=>true)]
    #[test_case('ン'=>false)]
    #[test_case('ﾝ'=>true)]
    fn is_half_katakana_works(c: char) -> bool {
        is_half_katakana(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('ア'=>false)]
    #[test_case('ｱ'=>false)]
    #[test_case('イ'=>false)]
    #[test_case('ｲ'=>false)]
    #[test_case('ヲ'=>false)]
    #[test_case('ｦ'=>false)]
    #[test_case('ン'=>false)]
    #[test_case('ﾝ'=>false)]
    #[test_case('、'=>true;"punctuation_dot")]
    #[test_case('。'=>true;"punctuation_circle")]
    fn is_punctuation_works(c: char) -> bool {
        is_punctuation(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('\n'=>false)]
    #[test_case('\r'=>false)]
    #[test_case('('=>false;"half_(")]
    #[test_case('（'=>false;"wide_(")]
    #[test_case('\t'=>true)]
    fn is_able_to_ruby_body_works(c: char) -> bool {
        is_able_to_ruby_body(c)
    }
}
