use super::*;
pub fn newline(input: &str) -> nom::IResult<&str, token::TokenKind> {
    match input.len().cmp(&1) {
        std::cmp::Ordering::Equal => {
            let c = input.bytes().next().unwrap();
            if c == b'\n' || c == b'\r' {
                Ok((&input[1..], token::TokenKind::NewLine))
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Char,
                )))
            }
        }
        std::cmp::Ordering::Greater => {
            let mut bytes = input.bytes();
            let first = bytes.next().unwrap();
            let second = bytes.next().unwrap();

            if first == b'\r' && second == b'\n' {
                Ok((&input[2..], token::TokenKind::NewLine))
            } else if (first == b'\r' && second != b'\n') || (first == b'\n') {
                Ok((&input[1..], token::TokenKind::NewLine))
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Char,
                )))
            }
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        ))),
    }
}
