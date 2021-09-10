use super::*;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
pub use nom::character::complete::*;
pub fn any_newline(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    let result: nom::IResult<token::Span, token::Span> = alt((
        nom::bytes::complete::tag("\n"),
        nom::bytes::complete::tag("\r\n"),
    ))(input);
    match result {
        Ok(ok) => Ok(ok),
        Err(_) => nom::bytes::complete::tag("\r")(input).map(|(input, parsed)| unsafe {
            (
                token::Span::new_from_raw_offset(
                    input.location_offset(),
                    input.location_line() + 1,
                    input.fragment(),
                    (),
                ),
                parsed,
            )
        }),
    }
}

pub fn kanji0(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while(is_kanji_related)(input)
}

pub fn kanji1(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while1(is_kanji_related)(input)
}

pub fn hiragana0(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while(is_hiragana)(input)
}

pub fn hiragana1(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while1(is_hiragana)(input)
}

pub fn katakana0(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while(is_katakana)(input)
}

pub fn katakana1(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while1(is_katakana)(input)
}

pub fn any_space0(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while(is_any_space)(input)
}

pub fn any_space1(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    take_while1(is_any_space)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("hoge"),
            nom::error::ErrorKind::Tag,
        ))))]
    #[test_case("hoge\nfoo"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("hoge\nfoo"),
            nom::error::ErrorKind::Tag,
        ))))]
    #[test_case("\n"=> Ok((token::test_helper::new_test_result_span(1, 2, ""),token::test_helper::new_test_result_span(0, 1, "\n"))))]
    #[test_case("\nhoge"=> Ok((token::test_helper::new_test_result_span(1, 2, "hoge"),token::test_helper::new_test_result_span(0, 1, "\n"))))]
    #[test_case("\n\n"=> Ok((token::test_helper::new_test_result_span(1, 2, "\n"),token::test_helper::new_test_result_span(0, 1, "\n"))))]
    #[test_case("\r"=> Ok((token::test_helper::new_test_result_span(1, 2, ""),token::test_helper::new_test_result_span(0, 1, "\r"))))]
    #[test_case("\rhoge"=> Ok((token::test_helper::new_test_result_span(1, 2, "hoge"),token::test_helper::new_test_result_span(0, 1, "\r"))))]
    #[test_case("\r\r"=> Ok((token::test_helper::new_test_result_span(1, 2, "\r"),token::test_helper::new_test_result_span(0, 1, "\r"))))]
    #[test_case("\r\n"=> Ok((token::test_helper::new_test_result_span(2, 2, ""),token::test_helper::new_test_result_span(0, 1, "\r\n"))))]
    #[test_case("\r\n\r\n"=> Ok((token::test_helper::new_test_result_span(2, 2, "\r\n"),token::test_helper::new_test_result_span(0, 1, "\r\n"))))]
    fn any_newline_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        any_newline(token::Span::new(input))
    }

    #[test_case("漢字"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),token::test_helper::new_test_result_span(0, 1, "漢字"))))]
    #[test_case("漢字とひらがな"=> Ok((token::test_helper::new_test_result_span(6, 1, "とひらがな"),token::test_helper::new_test_result_span(0, 1, "漢字"))))]
    #[test_case("なか漢字なか"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("なか漢字なか"),
            nom::error::ErrorKind::TakeWhile1,
        ))))]
    #[test_case("かんじなし"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("かんじなし"),
            nom::error::ErrorKind::TakeWhile1,
        ))))]
    fn kanji1_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        kanji1(token::Span::new(input))
    }
    #[test_case("漢字"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),token::test_helper::new_test_result_span(0, 1, "漢字"))))]
    #[test_case("邊󠄄"=> Ok((token::test_helper::new_test_result_span(7, 1, ""),token::test_helper::new_test_result_span(0, 1, "邊󠄄"))))]
    #[test_case("𠅘"=> Ok((token::test_helper::new_test_result_span(4, 1, ""),token::test_helper::new_test_result_span(0, 1, "𠅘"))))]
    #[test_case("漢字とひらがな"=> Ok((token::test_helper::new_test_result_span(6, 1, "とひらがな"),token::test_helper::new_test_result_span(0, 1, "漢字"))))]
    #[test_case("なか漢字なか"=> Ok((token::test_helper::new_test_result_span(0, 1, "なか漢字なか"),token::test_helper::new_test_result_span(0, 1, ""))))]
    #[test_case("かんじなし"=> Ok((token::test_helper::new_test_result_span(0, 1, "かんじなし"),token::test_helper::new_test_result_span(0, 1, ""))))]
    #[test_case("alphabet"=> Ok((token::test_helper::new_test_result_span(0, 1, "alphabet"),token::test_helper::new_test_result_span(0, 1, ""))))]
    #[test_case("01224"=> Ok((token::test_helper::new_test_result_span(0, 1, "01224"),token::test_helper::new_test_result_span(0, 1, ""))))]
    fn kanji0_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        kanji0(token::Span::new(input))
    }
    #[test_case("ひらがな"=> Ok((token::test_helper::new_test_result_span(12, 1, ""),token::test_helper::new_test_result_span(0, 1, "ひらがな"))))]
    #[test_case("ひらがなと漢字"=> Ok((token::test_helper::new_test_result_span(15, 1, "漢字"),token::test_helper::new_test_result_span(0, 1, "ひらがなと"))))]
    #[test_case("中ひらがな中"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("中ひらがな中"),
            nom::error::ErrorKind::TakeWhile1,
        ))))]
    #[test_case("漢字"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("漢字"),
            nom::error::ErrorKind::TakeWhile1,
        ))))]
    fn hiragana1_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        hiragana1(token::Span::new(input))
    }

    #[test_case("ひらがな"=> Ok((token::test_helper::new_test_result_span(12, 1, ""),token::test_helper::new_test_result_span(0, 1, "ひらがな"))))]
    #[test_case("ひらがなと漢字"=> Ok((token::test_helper::new_test_result_span(15, 1, "漢字"),token::test_helper::new_test_result_span(0, 1, "ひらがなと"))))]
    #[test_case("alphabet"=> Ok((token::test_helper::new_test_result_span(0, 1, "alphabet"),token::test_helper::new_test_result_span(0, 1, ""))))]
    #[test_case("01224"=> Ok((token::test_helper::new_test_result_span(0, 1, "01224"),token::test_helper::new_test_result_span(0, 1, ""))))]
    fn hiragana0_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        hiragana0(token::Span::new(input))
    }
    #[test_case("カタカナ"=> Ok((token::test_helper::new_test_result_span(12, 1, ""),token::test_helper::new_test_result_span(0, 1, "カタカナ"))))]
    #[test_case("カタカナと漢字"=> Ok((token::test_helper::new_test_result_span(12, 1, "と漢字"),token::test_helper::new_test_result_span(0, 1, "カタカナ"))))]
    #[test_case("中カタカナ中"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("中カタカナ中"),
            nom::error::ErrorKind::TakeWhile1,
        ))))]
    #[test_case("漢字"=> Err(nom::Err::Error(nom::error::Error::new(
            token::Span::new("漢字"),
            nom::error::ErrorKind::TakeWhile1,
        ))))]
    fn katakana1_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        katakana1(token::Span::new(input))
    }

    #[test_case("カタカナ"=> Ok((token::test_helper::new_test_result_span(12, 1, ""),token::test_helper::new_test_result_span(0, 1, "カタカナ"))))]
    #[test_case("カタカナと漢字"=> Ok((token::test_helper::new_test_result_span(12, 1, "と漢字"),token::test_helper::new_test_result_span(0, 1, "カタカナ"))))]
    #[test_case("alphabet"=> Ok((token::test_helper::new_test_result_span(0, 1, "alphabet"),token::test_helper::new_test_result_span(0, 1, ""))))]
    #[test_case("01224"=> Ok((token::test_helper::new_test_result_span(0, 1, "01224"),token::test_helper::new_test_result_span(0, 1, ""))))]
    fn katakana0_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        katakana0(token::Span::new(input))
    }
}
