pub mod complete;

pub use nom::character::is_alphabetic;
pub use nom::character::is_digit;

pub const fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '　'
}

pub const fn is_start_directive(c: char) -> bool {
    c == '|' || c == '｜'
}
pub const fn is_start_annotation(c: char) -> bool {
    c == '$' || c == '＄'
}

pub const fn is_end_annotation(c: char) -> bool {
    c == '$' || c == '＄'
}

pub const fn is_start_ruby(c: char) -> bool {
    c == '《'
}

pub const fn is_end_ruby(c: char) -> bool {
    c == '》'
}

pub const fn is_start_kanji_ruby(c: char) -> bool {
    c == '(' || c == '（'
}
pub const fn is_end_kanji_ruby(c: char) -> bool {
    c == ')' || c == '）'
}

pub use nom::character::is_hex_digit;

pub use nom::character::is_newline;
