use super::*;
use nom::bytes::complete::{take_while, take_while1, take_while_m_n};
use nom::sequence::{delimited, pair};
use nom_extend::character;
use nom_extend::character::complete;

pub fn newline(input: ParsedSpan) -> IResult {
    complete::any_newline(input).map(|(input, parsed)| (input, ParsedToken::NewLine(parsed)))
}

pub fn kanji(input: ParsedSpan) -> IResult {
    complete::kanji1(input).map(|(input, parsed)| (input, ParsedToken::Plaintext(parsed)))
}

const MAX_RUBY_COUNT_PER_BODY_CHAR: usize = 10;
const MAX_RUBY_COUNT_BODY: usize = 10;
pub fn kanji_ruby(input: ParsedSpan) -> IResult {
    let (input, body) = complete::kanji1(input)?;
    let mut ruby_parser = delimited(
        take_while_m_n(1, 1, character::is_start_ruby),
        complete::able_to_ruby,
        take_while_m_n(1, 1, character::is_end_ruby),
    );
    match ruby_parser(input) {
        Ok((forword_input, ruby)) => {
            let body_count = without_variation_selector_count(body.fragment());
            if body_count <= MAX_RUBY_COUNT_BODY {
                let ruby_count = without_variation_selector_count(ruby.fragment());
                if ruby_count <= MAX_RUBY_COUNT_PER_BODY_CHAR * body_count {
                    return Ok((forword_input, ParsedToken::KanjiRuby { body, ruby }));
                }
            }
            Ok((input, ParsedToken::Plaintext(body)))
        }
        Err(_) => Ok((input, ParsedToken::Plaintext(body))),
    }
}

pub fn space(input: ParsedSpan) -> IResult {
    complete::any_space1(input).map(|(input, parsed)| (input, ParsedToken::Space(parsed)))
}

pub fn plaintext(input: ParsedSpan) -> IResult {
    take_while1(character::is_plaintext)(input)
        .map(|(input, parsed)| (input, ParsedToken::Plaintext(parsed)))
}

pub fn directive_ruby(input: ParsedSpan) -> IResult {
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
        Ok((after_parsed_directive, ParsedToken::Ignore(directive)))
    } else {
        let body_count = without_variation_selector_count(body.fragment());
        let max_ruby_count = body_count * MAX_RUBY_COUNT_PER_BODY_CHAR;

        if without_variation_selector_count(ruby.fragment()) <= max_ruby_count {
            Ok((after_parsed_ruby, ParsedToken::Ruby { body, ruby }))
        } else {
            Ok((after_parsed_directive, ParsedToken::Plaintext(directive)))
        }
    }
}

pub fn emphasis_mark(input: ParsedSpan) -> IResult {
    delimited(
        complete::start_emphasis_mark,
        complete::able_to_emphasis_mark,
        complete::end_emphasis_mark,
    )(input)
    .map(|(input, parsed)| (input, ParsedToken::EmphasisMark(parsed)))
}

pub fn directive_other(input: ParsedSpan) -> IResult {
    take_while_m_n(1, 1, character::is_start_directive)(input)
        .map(|(input, parsed)| (input, ParsedToken::Plaintext(parsed)))
}

pub fn term_directive_other(input: ParsedSpan) -> IResult {
    take_while_m_n(1, 1, character::is_start_term)(input)
        .map(|(input, parsed)| (input, ParsedToken::Plaintext(parsed)))
}

pub fn emphasis_mark_start_other(input: ParsedSpan) -> IResult {
    take_while_m_n(1, 1, character::is_start_emphasis_mark)(input)
        .map(|(input, parsed)| (input, ParsedToken::Plaintext(parsed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "hoge"),nom::error::ErrorKind::Tag)))]
    #[test_case("\n"=> Ok((token::test_helper::new_test_result_span(1, 2, ""),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\n")))))]
    #[test_case("\nhoge"=> Ok((token::test_helper::new_test_result_span(1, 2, "hoge"),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\n")))))]
    #[test_case("\n\n"=> Ok((token::test_helper::new_test_result_span(1, 2, "\n"),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\n")))))]
    #[test_case("\r"=> Ok((token::test_helper::new_test_result_span(1, 2, ""),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\r")))))]
    #[test_case("\rhoge"=> Ok((token::test_helper::new_test_result_span(1, 2, "hoge"),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\r")))))]
    #[test_case("\r\r"=> Ok((token::test_helper::new_test_result_span(1, 2, "\r"),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\r")))))]
    #[test_case("\r\n"=> Ok((token::test_helper::new_test_result_span(2, 2, ""),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\r\n")))))]
    #[test_case("\r\n\r\n"=> Ok((token::test_helper::new_test_result_span(2, 2, "\r\n"),ParsedToken::NewLine(token::test_helper::new_test_result_span(0, 1, "\r\n")))))]
    fn newline_works(input: &str) -> IResult {
        newline(token::ParsedSpan::new(input))
    }

    #[test_case("??????"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "??????")))))]
    #[test_case("?????????????????????"=> Ok((token::test_helper::new_test_result_span(6, 1, "???????????????"),ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "??????")))))]
    #[test_case("??????????????????"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "??????????????????"),nom::error::ErrorKind::TakeWhile1)))]
    #[test_case("???????????????"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "???????????????"),nom::error::ErrorKind::TakeWhile1)))]
    fn kanji_works(input: &str) -> IResult {
        kanji(token::ParsedSpan::new(input))
    }

    #[test_case(" ???\t"=> Ok((token::test_helper::new_test_result_span(5, 1, ""),ParsedToken::Space(token::test_helper::new_test_result_span(0, 1, " ???\t")))))]
    #[test_case(" ???\t?????????????????????"=> Ok((token::test_helper::new_test_result_span(5, 1, "?????????????????????"),ParsedToken::Space(token::test_helper::new_test_result_span(0, 1, " ???\t")))))]
    #[test_case("??????????????????"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "??????????????????"),nom::error::ErrorKind::TakeWhile1)))]
    fn space_works(input: &str) -> IResult {
        space(token::ParsedSpan::new(input))
    }

    #[test_case("??????"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "??????")))))]
    #[test_case("??????|(?????????)"=> Ok((token::test_helper::new_test_result_span(6, 1, "|(?????????)"),ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "??????")))))]
    #[test_case("??????(?????????)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),ParsedToken::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "??????"),
    ruby:test_helper::new_test_result_span(7, 1, "?????????")}));"half")]
    #[test_case("?????????????????????????????????(?????????)"=> Ok((token::test_helper::new_test_result_span(33, 1, "(?????????)"),ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "?????????????????????????????????")))))]
    #[test_case("??????????????????????????????????????????????????????????????????????(??????????????????????????????)"=> Ok((token::test_helper::new_test_result_span(102, 1, ""),ParsedToken::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "??????????????????????????????????????????????????????????????????????"),
    ruby:test_helper::new_test_result_span(71, 1, "??????????????????????????????")}));"nabe")]
    #[test_case("?????????????????????"=> Ok((token::test_helper::new_test_result_span(21, 1, ""),ParsedToken::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "??????"),
    ruby:test_helper::new_test_result_span(9, 1, "?????????")}));"wide")]
    #[test_case("?????????????????????"=> Ok((token::test_helper::new_test_result_span(6, 1, "???????????????"),ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "??????")))))]
    #[test_case("????????????"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "????????????"),nom::error::ErrorKind::TakeWhile1)))]
    fn kanji_ruby_works(input: &str) -> IResult {
        kanji_ruby(token::ParsedSpan::new(input))
    }

    #[test_case("|??????(?????????)"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    ParsedToken::Ruby{
        body: token::test_helper::new_test_result_span(1, 1, "??????"),
        ruby: token::test_helper::new_test_result_span(8, 1, "?????????"),
    })))]
    #[test_case("|???????????????)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),
    ParsedToken::Ruby{
        body: token::test_helper::new_test_result_span(1, 1, "??????"),
        ruby: token::test_helper::new_test_result_span(10, 1, "??????"),
    })))]
    #[test_case("|??????(hoho???"=> Ok((token::test_helper::new_test_result_span(15, 1, ""),
    ParsedToken::Ruby{
        body: token::test_helper::new_test_result_span(1, 1, "??????"),
        ruby: token::test_helper::new_test_result_span(8, 1, "hoho"),
    })))]
    #[test_case("|(?????????)"=> Ok((token::test_helper::new_test_result_span(1, 1, "(?????????)"),ParsedToken::Ignore(token::test_helper::new_test_result_span(0, 1, "|"))));"half_directive")]
    #[test_case("???(?????????)"=> Ok((token::test_helper::new_test_result_span(3, 1, "(?????????)"),ParsedToken::Ignore(token::test_helper::new_test_result_span(0, 1, "???"))));"wide_directive")]
    fn directive_ruby_works(input: &str) -> IResult {
        directive_ruby(token::ParsedSpan::new(input))
    }

    #[test_case("????????????????????????"=> Ok((token::test_helper::new_test_result_span(24, 1, ""),ParsedToken::EmphasisMark(token::test_helper::new_test_result_span(6, 1, "????????????")))))]
    #[test_case("????????????\n????????????" => Err(new_error(token::test_helper::new_test_result_span(12, 1, "\n????????????"),nom::error::ErrorKind::TakeWhileMN)))]
    #[test_case("?????????not????????????" => Err(new_error(token::test_helper::new_test_result_span(3, 1, "??????not????????????"),nom::error::ErrorKind::TakeWhileMN)))]
    fn emphasis_mark_works(input: &str) -> IResult {
        emphasis_mark(token::ParsedSpan::new(input))
    }
}
