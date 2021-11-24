use super::*;
use nom::branch::alt;
use std::iter::FromIterator;

#[derive(new, Debug, PartialEq)]
pub struct TextIterator<'a> {
    context: ParseContext,
    input: ParsedSpan<'a>,
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = ParsedToken<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let (input, parsed) = alt((
            |input| self.context.term(input),
            |input| self.context.directive_annotation(input),
            complete::kanji_ruby,
            complete::directive_ruby,
            complete::directive_other,
            complete::term_directive_other,
            complete::space,
            complete::newline,
            complete::plaintext,
        ))(self.input)
        .ok()?;
        self.input = input;
        Some(parsed)
    }
}

impl<'a> FromIterator<ParsedToken<'a>> for TokenText {
    fn from_iter<I: IntoIterator<Item = ParsedToken<'a>>>(iter: I) -> Self {
        struct PlainSpanHolder {
            body: String,
            position: Position,
        }
        let mut plain_span = None;
        let mut tokens = vec![];
        for parsed_token in iter.into_iter() {
            match parsed_token {
                ParsedToken::Plaintext(body) => {
                    if plain_span.is_none() {
                        plain_span = Some(PlainSpanHolder {
                            body: String::new(),
                            position: Position::new(
                                body.location_line() as usize,
                                body.location_offset(),
                            ),
                        })
                    }
                    let mut old_plain_span = plain_span.unwrap();
                    old_plain_span.body.push_str(body.fragment());
                    plain_span = Some(old_plain_span);
                }
                _ => {
                    if let Some(span) = plain_span {
                        tokens.push(Token::Plaintext(Span::new(span.body, span.position)));
                        plain_span = None;
                    }
                    tokens.push(parsed_token.into());
                }
            }
        }
        if let Some(span) = plain_span {
            tokens.push(Token::Plaintext(Span::new(span.body, span.position)));
        }
        TokenText::new(tokens)
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
                Id::new("term_id1".into()),
                "穂積しょう".into(),
                "".into(),
                TokenText::new(vec![]),
            )]
        }

        pub fn other_terms() -> Vec<term::Term> {
            vec![term::Term::new(
                Id::new("term_id1".into()),
                "その他用語".into(),
                "".into(),
                TokenText::new(vec![]),
            )]
        }
    }

    #[test_case(token_works_testdata::hit_terms(),"穂積しょう" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("穂積しょう".into(),Position::new(1,0))),
            ],
            );"not_quote_term")]
    #[test_case(token_works_testdata::hit_terms(),"\"穂積しょう\"" => TokenText::new(
            vec![
                Token::new_term(Span::new("穂積しょう".into(),Position::new(1,1)),Id::new("term_id1".into())),
            ],
            );"quote_term")]
    #[test_case(token_works_testdata::hit_terms(),"穂積しょうたろう" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("穂積しょうたろう".into(),Position::new(1,0))),
            ]
            );"not_quote_after_sentence")]
    #[test_case(token_works_testdata::hit_terms(),"\"穂積しょう\"たろう" => TokenText::new(
            vec![
                Token::new_term(Span::new("穂積しょう".into(),Position::new(1,1)),Id::new("term_id1".into())),
                Token::new_plaintext(Span::new("たろう".into(),Position::new(1,17))),
            ],
            );"quote_after_sentence")]
    #[test_case(token_works_testdata::other_terms(),"\"穂積しょう\"たろう" =>TokenText::new(
            vec![
                Token::new_plaintext(Span::new("\"穂積しょう\"たろう".into(),Position::new(1,0))),
            ],
            ) )]
    #[test_case(token_works_testdata::hit_terms(),"|穂積《ほづみ》しょうたろう" => TokenText::new(
            vec![
                Token::new_ruby(Span::new("穂積".into(),Position::new(1,1)),Span::new("ほづみ".into(),Position::new(1,10))),
                Token::new_plaintext(Span::new("しょうたろう".into(),Position::new(1,22))),
            ],
            ))]
    #[test_case(token_works_testdata::hit_terms(),"穂積《ほづみ》しょうたろう" => TokenText::new(
            vec![
                Token::new_kanji_ruby(Span::new("穂積".into(),Position::new(1,0)),Span::new("ほづみ".into(),Position::new(1,9))),
                Token::new_plaintext(Span::new("しょうたろう".into(),Position::new(1,21))),
            ],
            );"kanji_ruby1")]
    #[test_case(token_works_testdata::hit_terms(),"穂積(ほづみ)しょうたろう" => TokenText::new(
            vec![
                Token::new_kanji_ruby(Span::new("穂積".into(),Position::new(1,0)),Span::new("ほづみ".into(),Position::new(1,7))),
                Token::new_plaintext(Span::new("しょうたろう".into(),Position::new(1,17))),
            ],
            );"kanji_ruby2")]
    #[test_case(token_works_testdata::hit_terms(),"|穂積しょうたろう" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("|穂積しょうたろう".into(),Position::new(1,0))),
            ],
            ))]
    #[test_case(token_works_testdata::other_terms(),"  スペース確認" => TokenText::new(
            vec![
                Token::new_spase(Span::new("  ".into(),Position::new(1,0))),
                Token::new_plaintext(Span::new("スペース確認".into(),Position::new(1,2))),
            ],
            ))]
    #[test_case(token_works_testdata::hit_terms(),"\"穂積しょう" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("\"穂積しょう".into(),Position::new(1,0))),
            ],
            );"part_quote_term")]
    #[test_case(token_works_testdata::hit_terms(),"穂積しょう\n\"穂積しょう\"" => TokenText::new(
            vec![
                Token::new_plaintext(Span::new("穂積しょう".into(),Position::new(1,0))),
                Token::new_new_line(Span::new("\n".into(),Position::new(1,15))),
                Token::new_term(Span::new("穂積しょう".into(),Position::new(2,17)),Id::new("term_id1".into())),
            ],
            );"new_line_with_term")]
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
