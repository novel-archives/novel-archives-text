use super::*;
use nom::branch::alt;
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
}
