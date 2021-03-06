use super::*;
use nom::InputTake;
use std::{
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

#[derive(new, Debug, PartialEq, Clone)]
pub struct TextIterator<'a> {
    context: ParseContext,
    input: ParsedSpan<'a>,
    #[new(default)]
    next_token: Box<Option<ParsedToken<'a>>>,
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if let Some(token) = self.next_token.deref() {
            let token = token.clone();
            *self.next_token.deref_mut() = None;
            Some(token)
        } else {
            let (input, parsed) = self.context.token(self.input).ok()?;
            match parsed {
                ParsedToken::Plaintext(span) => {
                    let mut input = input;
                    let mut len = span.fragment().len();
                    loop {
                        match self.context.token(input) {
                            Ok((new_input, token)) => match token {
                                ParsedToken::Plaintext(span) => {
                                    input = new_input;
                                    len += span.fragment().len();
                                }
                                _ => {
                                    let (_, parsed) = self.input.take_split(len);
                                    *self.next_token.deref_mut() = Some(token);
                                    self.input = new_input;
                                    return Some(ParsedToken::Plaintext(parsed));
                                }
                            },
                            Err(_) => {
                                let (input, parsed) = self.input.take_split(len);
                                self.input = input;
                                return Some(ParsedToken::Plaintext(parsed));
                            }
                        }
                    }
                }
                _ => {
                    self.input = input;
                    Some(parsed)
                }
            }
        }
    }
}

impl<'a> FromIterator<ParsedToken<'a>> for TokenText {
    fn from_iter<I: IntoIterator<Item = ParsedToken<'a>>>(iter: I) -> Self {
        TokenText::new(iter.into_iter().map(|token| token.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[cfg(test)]
    mod token_works_testdata {
        use super::*;
        pub fn hit_terms() -> Vec<term::Term> {
            vec![term::Term::new(
                Id::new("term_id1"),
                "???????????????".into(),
                "".into(),
                "".into(),
                false,
            )]
        }

        pub fn other_terms() -> Vec<term::Term> {
            vec![term::Term::new(
                Id::new("term_id1"),
                "???????????????".into(),
                "".into(),
                "".into(),
                false,
            )]
        }
    }

    #[test_case(token_works_testdata::hit_terms(),"???????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("???????????????".into(),Position::new(1,0))),
            ],
            );"not_quote_term")]
    #[test_case(token_works_testdata::hit_terms(),"\"???????????????\"" => TokenText::new(
            vec![
                Token::new_term(Span::new("???????????????".into(),Position::new(1,1)),Id::new("term_id1")),
            ],
            );"quote_term")]
    #[test_case(token_works_testdata::hit_terms(),"????????????????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("????????????????????????".into(),Position::new(1,0))),
            ]
            );"not_quote_after_sentence")]
    #[test_case(token_works_testdata::hit_terms(),"\"???????????????\"?????????" => TokenText::new(
            vec![
                Token::new_term(Span::new("???????????????".into(),Position::new(1,1)),Id::new("term_id1")),
                Token::new_plaintext(Span::new("?????????".into(),Position::new(1,17))),
            ],
            );"quote_after_sentence")]
    #[test_case(token_works_testdata::other_terms(),"\"???????????????\"?????????" =>TokenText::new(
            vec![
                Token::new_plaintext(Span::new("\"???????????????\"?????????".into(),Position::new(1,0))),
            ],
            ) )]
    #[test_case(token_works_testdata::hit_terms(),"|???????????????????????????????????????" => TokenText::new(
            vec![
                Token::new_ruby(Span::new("??????".into(),Position::new(1,1)),Span::new("?????????".into(),Position::new(1,10))),
                Token::new_plaintext(Span::new("??????????????????".into(),Position::new(1,22))),
            ],
            ))]
    #[test_case(token_works_testdata::hit_terms(),"???????????????????????????????????????" => TokenText::new(
            vec![
                Token::new_kanji_ruby(Span::new("??????".into(),Position::new(1,0)),Span::new("?????????".into(),Position::new(1,9))),
                Token::new_plaintext(Span::new("??????????????????".into(),Position::new(1,21))),
            ],
            );"kanji_ruby1")]
    #[test_case(token_works_testdata::hit_terms(),"??????(?????????)??????????????????" => TokenText::new(
            vec![
                Token::new_kanji_ruby(Span::new("??????".into(),Position::new(1,0)),Span::new("?????????".into(),Position::new(1,7))),
                Token::new_plaintext(Span::new("??????????????????".into(),Position::new(1,17))),
            ],
            );"kanji_ruby2")]
    #[test_case(token_works_testdata::hit_terms(),"|????????????????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("|????????????????????????".into(),Position::new(1,0))),
            ],
            ))]
    #[test_case(token_works_testdata::other_terms(),"  ??????????????????" => TokenText::new(
            vec![
                Token::new_spase(Span::new("  ".into(),Position::new(1,0))),
                Token::new_plaintext(Span::new("??????????????????".into(),Position::new(1,2))),
            ],
            ))]
    #[test_case(token_works_testdata::hit_terms(),"\"???????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("\"???????????????".into(),Position::new(1,0))),
            ],
            );"part_quote_term")]
    #[test_case(token_works_testdata::hit_terms(),"???????????????\n\"???????????????\"" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("???????????????".into(),Position::new(1,0))),
                Token::new_new_line(Span::new("\n".into(),Position::new(1,15))),
                Token::new_term(Span::new("???????????????".into(),Position::new(2,17)),Id::new("term_id1")),
            ],
            );"new_line_with_term")]
    #[test_case(token_works_testdata::other_terms(),"????????????????????????" => TokenText::new(
            vec![
                Token::new_emphasis_mark(Span::new("????????????".into(),Position::new(1,6))),
            ],
            ))]
    #[test_case(token_works_testdata::other_terms(),"?????????other??????????????????" => TokenText::new(
            vec![
                Token::new_emphasis_mark(Span::new("???other????????????".into(),Position::new(1,6))),
            ],
            ))]
    #[test_case(token_works_testdata::other_terms(),"??????not???????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("??????not???????????????".into(),Position::new(1,0))),
            ],
            ))]
    #[test_case(token_works_testdata::other_terms(),"???not?????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("???not?????????".into(),Position::new(1,0))),
            ],
            );"not_emphasis_1")]
    #[test_case(token_works_testdata::other_terms(),"?????????not????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("?????????not????????????".into(),Position::new(1,0))),
            ],
            );"not_emphasis_2")]
    #[test_case(token_works_testdata::other_terms(),"???????????????\n?????????????????????????????????" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("???????????????".into(),Position::new(1,0))),
                Token::new_new_line(Span::new("\n".into(),Position::new(1,15))),
                Token::new_plaintext(Span::new("?????????".into(),Position::new(2,16))),
                Token::new_kanji_ruby(Span::new("???".into(),Position::new(2,25)),Span::new("????????????".into(),Position::new(2,31))),
                Token::new_plaintext(Span::new("???".into(),Position::new(2,46))),
            ],
            ))]
    fn context_token_works(terms: Vec<term::Term>, input: &str) -> TokenText {
        let iter = TextIterator::new(
            ParseContext::new(Arc::new(
                terms
                    .into_iter()
                    .map(|term| (term.body().clone(), term))
                    .collect(),
            )),
            token::ParsedSpan::new(input),
        );
        iter.collect()
    }
}
