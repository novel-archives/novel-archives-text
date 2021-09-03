use super::*;
use nom::branch::alt;
pub use nom::character::complete::*;
pub fn newline(input: token::Span) -> nom::IResult<token::Span, token::Span> {
    alt((
        nom::bytes::complete::tag("\n"),
        nom::bytes::complete::tag("\r\n"),
        nom::bytes::complete::tag("\r"),
    ))(input)
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
    #[test_case("\n"=> Ok((new_test_result_span(1, 2, ""),new_test_result_span(0, 1, "\n"))))]
    #[test_case("\nhoge"=> Ok((new_test_result_span(1, 2, "hoge"),new_test_result_span(0, 1, "\n"))))]
    #[test_case("\n\n"=> Ok((new_test_result_span(1, 2, "\n"),new_test_result_span(0, 1, "\n"))))]
    #[test_case("\r"=> Ok((new_test_result_span(1, 1, ""),new_test_result_span(0, 1, "\r"))))]
    #[test_case("\rhoge"=> Ok((new_test_result_span(1, 1, "hoge"),new_test_result_span(0, 1, "\r"))))]
    #[test_case("\r\r"=> Ok((new_test_result_span(1, 1, "\r"),new_test_result_span(0, 1, "\r"))))]
    #[test_case("\r\n"=> Ok((new_test_result_span(2, 2, ""),new_test_result_span(0, 1, "\r\n"))))]
    #[test_case("\r\n\r\n"=> Ok((new_test_result_span(2, 2, "\r\n"),new_test_result_span(0, 1, "\r\n"))))]
    fn newline_works(input: &str) -> nom::IResult<token::Span, token::Span> {
        newline(token::Span::new(input))
    }

    fn new_test_result_span(offset: usize, line: u32, fragment: &str) -> token::Span {
        unsafe { token::Span::new_from_raw_offset(offset, line, fragment, ()) }
    }
}
