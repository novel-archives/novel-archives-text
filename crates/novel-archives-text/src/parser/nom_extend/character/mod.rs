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
    c == '《' || c == '⟪'
}

pub fn is_end_ruby(c: char) -> bool {
    c == '》' || c == '⟫'
}

pub fn is_start_kanji_ruby(c: char) -> bool {
    c == '(' || c == '（'
}
pub fn is_end_kanji_ruby(c: char) -> bool {
    c == ')' || c == '）'
}

#[allow(clippy::manual_range_contains)]
pub fn is_kanji(c: char) -> bool {
    kanji::is_kanji(&c) || (c >= '\u{e0100}' && c <= '\u{e01ef}')
}

pub fn is_hiragana(c: char) -> bool {
    kanji::is_hiragana(&c)
}

pub fn is_katakana(c: char) -> bool {
    kanji::is_katakana(&c)
}

#[allow(clippy::manual_range_contains)]
pub fn is_zenkaku_alphabetic(c: char) -> bool {
    (c >= 'ａ' && c <= 'ｚ') || (c >= 'Ａ' && c <= 'Ｚ')
}

#[allow(clippy::manual_range_contains)]
pub fn is_zenkaku_hankaku_alphabetic(c: char) -> bool {
    is_zenkaku_alphabetic(c) || ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'))
}

#[allow(clippy::manual_range_contains)]
pub fn is_zenkaku_disit(c: char) -> bool {
    c >= '０' && c <= '９'
}

#[allow(clippy::manual_range_contains)]
pub fn is_zenkaku_hankaku_disit(c: char) -> bool {
    is_zenkaku_disit(c) || (c >= '0' && c <= '9')
}

pub fn is_start_link_annotation(c: char) -> bool {
    c == '*' || c == '＊'
}

pub fn is_able_to_ruby(c: char) -> bool {
    is_any_space(c)
        || is_zenkaku_hankaku_disit(c)
        || is_kanji(c)
        || is_hiragana(c)
        || is_katakana(c)
        || is_zenkaku_hankaku_alphabetic(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[allow(clippy::bool_assert_comparison)]
    #[test_case(' '=>true;"hankaku_space")]
    #[test_case('　'=>true;"zenkaku_space")]
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
    #[test_case('|'=>true;"hankaku_pipe")]
    #[test_case('｜'=>true;"zenkaku_pipe")]
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
    #[test_case('$'=>true;"hankaku_$")]
    #[test_case('＄'=>true;"zenkaku_＄")]
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
    #[test_case('$'=>true;"hankaku_$")]
    #[test_case('＄'=>true;"zenkaku_＄")]
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
    #[test_case('('=>true;"hankaku")]
    #[test_case('（'=>true;"zenkaku")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_start_kanji_ruby_works(c: char) -> bool {
        is_start_kanji_ruby(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case(')'=>true;"hankaku")]
    #[test_case('）'=>true;"zenkaku")]
    #[test_case('a'=>false)]
    #[test_case('b'=>false)]
    #[test_case('0'=>false)]
    #[test_case('０'=>false)]
    #[test_case('9'=>false)]
    #[test_case('９'=>false)]
    #[test_case('あ'=>false)]
    #[test_case('塡'=>false)]
    fn is_end_kanji_ruby_works(c: char) -> bool {
        is_end_kanji_ruby(c)
    }
    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    fn is_zenkaku_alphabetic_works(c: char) -> bool {
        is_zenkaku_alphabetic(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    fn is_zenkaku_hankaku_alphabetic_works(c: char) -> bool {
        is_zenkaku_hankaku_alphabetic(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    fn is_zenkaku_disit_works(c: char) -> bool {
        is_zenkaku_disit(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
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
    fn is_zenkaku_hankaku_disit_works(c: char) -> bool {
        is_zenkaku_hankaku_disit(c)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case('('=>false;"hankaku")]
    #[test_case('（'=>false;"zenkaku")]
    #[test_case('*'=>true;"hankaku_link_annotation")]
    #[test_case('＊'=>true;"zenkaku_link_annotation")]
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
}
