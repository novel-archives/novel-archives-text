use super::*;
use nom::bytes::complete::{take_while, take_while1, take_while_m_n};
use nom::sequence::{delimited, pair, tuple};
use nom_extend::character;
use nom_extend::character::complete;

pub fn newline(input: Span) -> IResult {
    complete::any_newline(input).map(|(input, parsed)| (input, Token::NewLine(parsed)))
}

const MAX_RUBY_COUNT_PER_BODY_CHAR: usize = 10;
const MAX_RUBY_COUNT_BODY: usize = 10;

pub fn kanji_ruby(input: Span) -> IResult {
    let (input, body) = complete::kanji1(input)?;
    let mut ruby_parser = delimited(
        take_while_m_n(1, 1, character::is_start_ruby),
        complete::able_to_ruby,
        take_while_m_n(1, 1, character::is_end_ruby),
    );
    let result = ruby_parser(input);
    match result {
        Ok((forword_input, ruby)) => {
            let body_count = without_variation_selector_count(body.fragment());
            if body_count <= MAX_RUBY_COUNT_BODY {
                let ruby_count = without_variation_selector_count(ruby.fragment());
                if ruby_count <= MAX_RUBY_COUNT_PER_BODY_CHAR * body_count {
                    return Ok((
                        forword_input,
                        Token::KanjiRuby {
                            body,
                            ruby: iterator::RubyIterator::new(ruby),
                        },
                    ));
                }
            }
            Ok((input, Token::Kanji(body)))
        }
        Err(_) => Ok((input, Token::Kanji(body))),
    }
}

pub fn directive_ruby(input: Span) -> IResult {
    let (after_parsed_directive, directive) = complete::start_directive(input)?;
    let (after_parsed_ruby, (body, ruby)) = pair(
        take_while(character::is_able_to_ruby_body),
        delimited(
            take_while_m_n(1, 1, character::is_start_ruby),
            complete::able_to_ruby,
            take_while_m_n(1, 1, character::is_end_ruby),
        ),
    )(after_parsed_directive)?;
    if body.fragment().is_empty() {
        Ok((after_parsed_directive, Token::Ignore(directive)))
    } else {
        let body_count = without_variation_selector_count(body.fragment());
        let max_ruby_count = body_count * MAX_RUBY_COUNT_PER_BODY_CHAR;

        if without_variation_selector_count(ruby.fragment()) <= max_ruby_count {
            Ok((
                after_parsed_ruby,
                Token::Ruby {
                    body: iterator::RubyBodyIterator::new(body),
                    ruby: iterator::RubyIterator::new(ruby),
                },
            ))
        } else {
            Ok((after_parsed_directive, Token::Other(directive)))
        }
    }
}

pub fn directive_annotation(input: Span) -> IResult {
    tuple((
        complete::start_directive,
        take_while1(character::is_able_to_annotation_body),
        delimited(
            take_while_m_n(1, 1, character::is_start_annotation),
            complete::able_to_annotation,
            take_while_m_n(1, 1, character::is_end_annotation),
        ),
    ))(input)
    .map(|(input, (_, body, description))| {
        (
            input,
            Token::Annotation {
                body: iterator::AnnotationBodyIterator::new(body),
                description: iterator::AnnotationDescriptionIterator::new(description),
            },
        )
    })
}

pub fn kanji(input: Span) -> IResult {
    complete::kanji1(input).map(|(input, parsed)| (input, Token::Kanji(parsed)))
}

fn without_variation_selector_count(input: &str) -> usize {
    input
        .chars()
        .filter(|&c| !character::is_kanji_variation_selector(c))
        .count()
}

pub fn hiragana(input: Span) -> IResult {
    complete::hiragana1(input).map(|(input, parsed)| (input, Token::Hiragana(parsed)))
}

pub fn katakana(input: Span) -> IResult {
    complete::katakana1(input).map(|(input, parsed)| (input, Token::Katakana(parsed)))
}

pub fn half_and_wide_disit(input: Span) -> IResult {
    half_and_wide_usize(input).map(|(input, (parsed, digit))| {
        (
            input,
            Token::Digit {
                body: parsed,
                digit,
            },
        )
    })
}

pub fn wide_alphabet(input: Span) -> IResult {
    complete::wide_alphabetic1(input).map(|(input, parsed)| (input, Token::WideAlphabet(parsed)))
}

pub fn space(input: Span) -> IResult {
    complete::any_space1(input).map(|(input, parsed)| (input, Token::Space(parsed)))
}

pub fn half_and_wide_usize(input: Span) -> IResult<(Span, usize)> {
    let (input, parsed) = take_while1(character::is_wide_half_disit)(input)?;
    Ok((
        input,
        (
            parsed,
            parsed
                .chars()
                .map(character::wide_half_disit_char_to_disit)
                .map(|o| o.unwrap() as usize)
                .fold(Some(0_usize), |s, v| s?.checked_mul(10)?.checked_add(v))
                .ok_or_else(|| new_error(parsed, nom::error::ErrorKind::Digit))?,
        ),
    ))
}

pub fn half_katakana(input: Span) -> IResult {
    complete::half_katakana1(input).map(|(input, parsed)| (input, Token::HalfKatakana(parsed)))
}

pub fn punctuation(input: Span) -> IResult {
    complete::punctuation1(input).map(|(input, parsed)| (input, Token::Punctuation(parsed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "hoge"),nom::error::ErrorKind::Tag)))]
    #[test_case("\n"=> Ok((token::test_helper::new_test_result_span(1, 2, ""),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\n")))))]
    #[test_case("\nhoge"=> Ok((token::test_helper::new_test_result_span(1, 2, "hoge"),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\n")))))]
    #[test_case("\n\n"=> Ok((token::test_helper::new_test_result_span(1, 2, "\n"),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\n")))))]
    #[test_case("\r"=> Ok((token::test_helper::new_test_result_span(1, 2, ""),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\r")))))]
    #[test_case("\rhoge"=> Ok((token::test_helper::new_test_result_span(1, 2, "hoge"),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\r")))))]
    #[test_case("\r\r"=> Ok((token::test_helper::new_test_result_span(1, 2, "\r"),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\r")))))]
    #[test_case("\r\n"=> Ok((token::test_helper::new_test_result_span(2, 2, ""),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\r\n")))))]
    #[test_case("\r\n\r\n"=> Ok((token::test_helper::new_test_result_span(2, 2, "\r\n"),Token::NewLine(token::test_helper::new_test_result_span(0, 1, "\r\n")))))]
    fn newline_works(input: &str) -> IResult {
        newline(token::Span::new(input))
    }

    #[test_case("漢字"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("漢字とひらがな"=> Ok((token::test_helper::new_test_result_span(6, 1, "とひらがな"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("なか漢字なか"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "なか漢字なか"),nom::error::ErrorKind::TakeWhile1)))]
    #[test_case("かんじなし"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "かんじなし"),nom::error::ErrorKind::TakeWhile1)))]
    fn kanji_works(input: &str) -> IResult {
        kanji(token::Span::new(input))
    }

    #[test_case("ひらがな"=> Ok((token::test_helper::new_test_result_span(12, 1, ""),Token::Hiragana(token::test_helper::new_test_result_span(0, 1, "ひらがな")))))]
    #[test_case("ひらがなと漢字"=> Ok((token::test_helper::new_test_result_span(15, 1, "漢字"),Token::Hiragana(token::test_helper::new_test_result_span(0, 1, "ひらがなと")))))]
    #[test_case("中ひらがな中"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "中ひらがな中"),nom::error::ErrorKind::TakeWhile1)))]
    fn hiragana_works(input: &str) -> IResult {
        hiragana(token::Span::new(input))
    }

    #[test_case("カタカナ"=> Ok((token::test_helper::new_test_result_span(12, 1, ""),Token::Katakana(token::test_helper::new_test_result_span(0, 1, "カタカナ")))))]
    #[test_case("カタカナと漢字"=> Ok((token::test_helper::new_test_result_span(12, 1, "と漢字"),Token::Katakana(token::test_helper::new_test_result_span(0, 1, "カタカナ")))))]
    #[test_case("中カタカナ中"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "中カタカナ中"),nom::error::ErrorKind::TakeWhile1)))]
    fn katakana_works(input: &str) -> IResult {
        katakana(token::Span::new(input))
    }

    #[test_case("１３32"=> Ok((token::test_helper::new_test_result_span(8, 1, ""),Token::Digit{body:token::test_helper::new_test_result_span(0, 1, "１３32"),digit:1332})))]
    #[test_case("１３32ほげ"=> Ok((token::test_helper::new_test_result_span(8, 1, "ほげ"),Token::Digit{body:token::test_helper::new_test_result_span(0, 1, "１３32"),digit:1332})))]
    #[test_case("ふが"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "ふが"),nom::error::ErrorKind::TakeWhile1)))]
    #[test_case("999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999"
        => Err(new_error(token::test_helper::new_test_result_span(0, 1, "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999"),nom::error::ErrorKind::Digit)))]
    fn half_and_wide_disit_works(input: &str) -> IResult {
        half_and_wide_disit(token::Span::new(input))
    }

    #[test_case(" 　\t"=> Ok((token::test_helper::new_test_result_span(5, 1, ""),Token::Space(token::test_helper::new_test_result_span(0, 1, " 　\t")))))]
    #[test_case(" 　\tカタカナと漢字"=> Ok((token::test_helper::new_test_result_span(5, 1, "カタカナと漢字"),Token::Space(token::test_helper::new_test_result_span(0, 1, " 　\t")))))]
    #[test_case("中カタカナ中"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "中カタカナ中"),nom::error::ErrorKind::TakeWhile1)))]
    fn space_works(input: &str) -> IResult {
        space(token::Span::new(input))
    }

    #[test_case("ｓｃｄ"=> Ok((token::test_helper::new_test_result_span(9, 1, ""),Token::WideAlphabet(token::test_helper::new_test_result_span(0, 1, "ｓｃｄ")))))]
    #[test_case("ｓｃｄと漢字"=> Ok((token::test_helper::new_test_result_span(9, 1, "と漢字"),Token::WideAlphabet(token::test_helper::new_test_result_span(0, 1, "ｓｃｄ")))))]
    #[test_case("中カタカナ中"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "中カタカナ中"),nom::error::ErrorKind::TakeWhile1)))]
    fn wide_alphabet_works(input: &str) -> IResult {
        wide_alphabet(token::Span::new(input))
    }

    #[test_case("ｱｲｳｴｵ"=> Ok((token::test_helper::new_test_result_span(15, 1, ""),Token::HalfKatakana(token::test_helper::new_test_result_span(0, 1, "ｱｲｳｴｵ")))))]
    #[test_case("ｱｲｳｴｵアイウエオ"=> Ok((token::test_helper::new_test_result_span(15, 1, "アイウエオ"),Token::HalfKatakana(token::test_helper::new_test_result_span(0, 1, "ｱｲｳｴｵ")))))]
    #[test_case("中カタカナ中"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "中カタカナ中"),nom::error::ErrorKind::TakeWhile1)))]
    fn half_katakana_works(input: &str) -> IResult {
        half_katakana(token::Span::new(input))
    }

    #[test_case("漢字"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("漢字|(かんじ)"=> Ok((token::test_helper::new_test_result_span(6, 1, "|(かんじ)"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("漢字(かんじ)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),Token::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "漢字"),
    ruby:iterator::RubyIterator::new(test_helper::new_test_result_span(7, 1, "かんじ"))}));"half")]
    #[test_case("漢字漢字漢字漢字漢字字(かんじ)"=> Ok((token::test_helper::new_test_result_span(33, 1, "(かんじ)"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字漢字漢字漢字漢字字")))))]
    #[test_case("邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄(なべなべなべなべなべ)"=> Ok((token::test_helper::new_test_result_span(102, 1, ""),Token::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄"),
    ruby:iterator::RubyIterator::new(test_helper::new_test_result_span(71, 1, "なべなべなべなべなべ"))}));"nabe")]
    #[test_case("漢字（かんじ）"=> Ok((token::test_helper::new_test_result_span(21, 1, ""),Token::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "漢字"),
    ruby:iterator::RubyIterator::new(test_helper::new_test_result_span(9, 1, "かんじ"))}));"wide")]
    #[test_case("漢字アイウエオ"=> Ok((token::test_helper::new_test_result_span(6, 1, "アイウエオ"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("カタカナ"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "カタカナ"),nom::error::ErrorKind::TakeWhile1)))]
    fn kanji_ruby_works(input: &str) -> IResult {
        kanji_ruby(token::Span::new(input))
    }

    #[test_case("。ｱｲｳｴｵ"=> Ok((token::test_helper::new_test_result_span(3, 1, "ｱｲｳｴｵ"),Token::Punctuation(token::test_helper::new_test_result_span(0, 1, "。"))));"punctuation_circle")]
    #[test_case("、ｱｲｳｴｵ"=> Ok((token::test_helper::new_test_result_span(3, 1, "ｱｲｳｴｵ"),Token::Punctuation(token::test_helper::new_test_result_span(0, 1, "、"))));"punctuation_dot")]
    #[test_case("中カタカナ中"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "中カタカナ中"),nom::error::ErrorKind::TakeWhile1)))]
    fn punctuation_works(input: &str) -> IResult {
        punctuation(token::Span::new(input))
    }

    #[test_case("|漢字(かんじ)"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    Token::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字")),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ")),
    })))]
    #[test_case("|ほげ（ふが)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),
    Token::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "ほげ")),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(10, 1, "ふが")),
    })))]
    #[test_case("|ふ符(hoho）"=> Ok((token::test_helper::new_test_result_span(15, 1, ""),
    Token::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "ふ符")),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(8, 1, "hoho")),
    })))]
    #[test_case("|(かんじ)"=> Ok((token::test_helper::new_test_result_span(1, 1, "(かんじ)"),Token::Ignore(token::test_helper::new_test_result_span(0, 1, "|"))));"half_directive")]
    #[test_case("｜(かんじ)"=> Ok((token::test_helper::new_test_result_span(3, 1, "(かんじ)"),Token::Ignore(token::test_helper::new_test_result_span(0, 1, "｜"))));"wide_directive")]
    fn directive_ruby_works(input: &str) -> IResult {
        directive_ruby(token::Span::new(input))
    }

    #[test_case("|漢字$かんじ$"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字")),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ")),
    }));"half_all")]
    #[test_case("|漢字(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(32, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字(かんじ)")),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(19, 1, "せつめい")),
    }));"with_ruby")]
    #[test_case("||漢字ふ(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(36, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "|漢字ふ(かんじ)")),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(23, 1, "せつめい")),
    }));"with_ruby_directive")]
    #[test_case("|漢字＄かんじ$"=> Ok((token::test_helper::new_test_result_span(20, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字")),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(10, 1, "かんじ")),
    }));"wide_start")]
    #[test_case("|$hoge$"=> Err(new_error(token::test_helper::new_test_result_span(1, 1, "$hoge$"),nom::error::ErrorKind::TakeWhile1)))]
    fn directive_annotation_works(input: &str) -> IResult {
        directive_annotation(token::Span::new(input))
    }
}
