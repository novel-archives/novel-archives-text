use nom::bytes::complete::{take_while, take_while1, take_while_m_n};
use nom::sequence::{delimited, pair, tuple};
use nom_extend::character::complete;
use std::cmp::Reverse;

use super::*;
use nom::InputTake;

#[derive(Debug, PartialEq, Clone, new)]
pub struct Context {
    term_map: Arc<TermMap>,
}

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct TermKv {
    key: String,
    term: Arc<term::Term>,
}

impl Context {
    const MAX_RUBY_COUNT_PER_BODY_CHAR: usize = 10;
    const MAX_RUBY_COUNT_BODY: usize = 10;
    pub fn term<'a>(&self, input: Span<'a>) -> IResult<'a> {
        let fragment = input.fragment();
        let term_map = &self.term_map.0;
        let first = fragment
            .chars()
            .next()
            .ok_or_else(|| new_error(input, nom::error::ErrorKind::TakeTill1))?;
        let (_, terms) = term_map
            .get(
                term_map
                    .binary_search_by_key(&first, |(k, _)| *k)
                    .map_err(|_| new_error(input, nom::error::ErrorKind::TakeTill1))?,
            )
            .unwrap();

        let term = terms
            .iter()
            .find(|term| term.key().len() <= fragment.len() && fragment.starts_with(term.key()))
            .ok_or_else(|| new_error(input, nom::error::ErrorKind::TakeTill1))?;

        let (input, body) = input.take_split(term.key().len());

        Ok((
            input,
            Token::Term {
                body,
                term: term.term().clone(),
            },
        ))
    }

    pub fn kanji_ruby<'a>(&self, input: Span<'a>) -> IResult<'a> {
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
                if body_count <= Self::MAX_RUBY_COUNT_BODY {
                    let ruby_count = without_variation_selector_count(ruby.fragment());
                    if ruby_count <= Self::MAX_RUBY_COUNT_PER_BODY_CHAR * body_count {
                        return Ok((
                            forword_input,
                            Token::KanjiRuby {
                                body,
                                ruby: iterator::RubyIterator::new(ruby, self.clone()),
                            },
                        ));
                    }
                }
                Ok((input, Token::Kanji(body)))
            }
            Err(_) => Ok((input, Token::Kanji(body))),
        }
    }

    pub fn directive_ruby<'a>(&self, input: Span<'a>) -> IResult<'a> {
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
            let max_ruby_count = body_count * Self::MAX_RUBY_COUNT_PER_BODY_CHAR;

            if without_variation_selector_count(ruby.fragment()) <= max_ruby_count {
                Ok((
                    after_parsed_ruby,
                    Token::Ruby {
                        body: iterator::RubyBodyIterator::new(body, self.clone()),
                        ruby: iterator::RubyIterator::new(ruby, self.clone()),
                    },
                ))
            } else {
                Ok((after_parsed_directive, Token::Other(directive)))
            }
        }
    }

    pub fn directive_annotation<'a>(&self, input: Span<'a>) -> IResult<'a> {
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
                    body: iterator::AnnotationBodyIterator::new(body, self.clone()),
                    description: iterator::AnnotationDescriptionIterator::new(
                        description,
                        self.clone(),
                    ),
                },
            )
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TermMap(Vec<(char, Vec<TermKv>)>);

impl TermMap {
    pub fn new(terms: &[term::Term]) -> Self {
        let term_kvs: Vec<_> = terms
            .iter()
            .map(|t| TermKv::new(t.body().to_string(), Arc::new(t.clone())))
            .collect();
        let mut term_map = Self(Vec::with_capacity(term_kvs.len()));

        for term_kv in term_kvs.iter() {
            let ck = term_kv.key().chars().next().unwrap();
            match term_map.0.binary_search_by_key(&ck, |(k, _)| *k) {
                Ok(i) => {
                    let (_, terms) = term_map.0.get_mut(i).unwrap();
                    terms.push(term_kv.clone());
                }
                Err(_) => {
                    term_map.0.push((ck, vec![term_kv.clone()]));
                    term_map.0.sort_by_key(|(k, _)| *k);
                }
            }
        }
        for (_, terms) in term_map.0.iter_mut() {
            terms.sort_by_key(|kv| Reverse(without_variation_selector_count(kv.key())));
        }
        term_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn new_sample_term(text: &str) -> term::Term {
        term::Term::new(
            Id::new("term_id1".into()),
            TokenText::new(vec![TokenKind::new_kanji(crate::token::Token::new(
                text.into(),
                Position::new(0, 0),
            ))]),
            TokenText::new(vec![TokenKind::new_hiragana(crate::token::Token::new(
                "む".into(),
                Position::new(0, 0),
            ))]),
            TokenText::new(vec![]),
        )
    }
    #[test_case(TermMap::new(&[
            new_sample_term("無"),
    ]),"無"
        =>Ok((token::test_helper::new_test_result_span(3, 1, ""),
        Token::Term{
            body:token::test_helper::new_test_result_span(0, 1, "無"),
            term:Arc::new(new_sample_term("無")),
        }))
    )]
    #[test_case(TermMap::new(&[
            new_sample_term("穂積"),
    ]),"穂積しょう"
        =>Ok((token::test_helper::new_test_result_span(6, 1, "しょう"),
        Token::Term{
            body:token::test_helper::new_test_result_span(0, 1, "穂積"),
            term:Arc::new(new_sample_term("穂積")),
        }))
    )]
    #[test_case(TermMap::new(&[
            new_sample_term("穂積"),
    ]),"しょう"
        =>Err(new_error(token::test_helper::new_test_result_span(0, 1, "しょう"),nom::error::ErrorKind::TakeTill1))
    )]
    #[test_case(TermMap::new(&[
            new_sample_term("穂積"),
    ]),"穂"
        =>Err(new_error(token::test_helper::new_test_result_span(0, 1, "穂"),nom::error::ErrorKind::TakeTill1))
    )]
    #[test_case(TermMap::new(&[
            new_sample_term("穂積"),
            new_sample_term("穂積しょう"),
    ]),"穂積しょう"
        =>Ok((token::test_helper::new_test_result_span(15, 1, ""),
        Token::Term{
            body:token::test_helper::new_test_result_span(0, 1, "穂積しょう"),
            term:Arc::new(new_sample_term("穂積しょう")),
        }))
    )]
    fn context_term_works(term_map: TermMap, input: &str) -> IResult {
        let ctx = Context::new(Arc::new(term_map));
        ctx.term(token::Span::new(input))
    }

    fn default_ctx() -> Context {
        Context::new(Arc::new(TermMap::new(&[])))
    }

    #[test_case("漢字"=> Ok((token::test_helper::new_test_result_span(6, 1, ""),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("漢字|(かんじ)"=> Ok((token::test_helper::new_test_result_span(6, 1, "|(かんじ)"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("漢字(かんじ)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),Token::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "漢字"),
    ruby:iterator::RubyIterator::new(test_helper::new_test_result_span(7, 1, "かんじ"),default_ctx())}));"half")]
    #[test_case("漢字漢字漢字漢字漢字字(かんじ)"=> Ok((token::test_helper::new_test_result_span(33, 1, "(かんじ)"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字漢字漢字漢字漢字字")))))]
    #[test_case("邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄(なべなべなべなべなべ)"=> Ok((token::test_helper::new_test_result_span(102, 1, ""),Token::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄邊󠄄"),
    ruby:iterator::RubyIterator::new(test_helper::new_test_result_span(71, 1, "なべなべなべなべなべ"),default_ctx())}));"nabe")]
    #[test_case("漢字（かんじ）"=> Ok((token::test_helper::new_test_result_span(21, 1, ""),Token::KanjiRuby{body:token::test_helper::new_test_result_span(0, 1, "漢字"),
    ruby:iterator::RubyIterator::new(test_helper::new_test_result_span(9, 1, "かんじ"),Context::new(Arc::new(TermMap::new(&[]))))}));"wide")]
    #[test_case("漢字アイウエオ"=> Ok((token::test_helper::new_test_result_span(6, 1, "アイウエオ"),Token::Kanji(token::test_helper::new_test_result_span(0, 1, "漢字")))))]
    #[test_case("カタカナ"=> Err(new_error(token::test_helper::new_test_result_span(0, 1, "カタカナ"),nom::error::ErrorKind::TakeWhile1)))]
    fn context_kanji_ruby_works(input: &str) -> IResult {
        default_ctx().kanji_ruby(token::Span::new(input))
    }

    #[test_case("|漢字(かんじ)"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    Token::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字"),default_ctx()),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ"),default_ctx()),
    })))]
    #[test_case("|ほげ（ふが)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),
    Token::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "ほげ"),default_ctx()),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(10, 1, "ふが"),default_ctx()),
    })))]
    #[test_case("|ふ符(hoho）"=> Ok((token::test_helper::new_test_result_span(15, 1, ""),
    Token::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "ふ符"),default_ctx()),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(8, 1, "hoho"),default_ctx()),
    })))]
    #[test_case("|(かんじ)"=> Ok((token::test_helper::new_test_result_span(1, 1, "(かんじ)"),Token::Ignore(token::test_helper::new_test_result_span(0, 1, "|"))));"half_directive")]
    #[test_case("｜(かんじ)"=> Ok((token::test_helper::new_test_result_span(3, 1, "(かんじ)"),Token::Ignore(token::test_helper::new_test_result_span(0, 1, "｜"))));"wide_directive")]
    fn directive_ruby_works(input: &str) -> IResult {
        default_ctx().directive_ruby(token::Span::new(input))
    }

    #[test_case("|漢字$かんじ$"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ"),default_ctx()),
    }));"half_all")]
    #[test_case("|漢字(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(32, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字(かんじ)"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(19, 1, "せつめい"),default_ctx()),
    }));"with_ruby")]
    #[test_case("||漢字ふ(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(36, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "|漢字ふ(かんじ)"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(23, 1, "せつめい"),default_ctx()),
    }));"with_ruby_directive")]
    #[test_case("|漢字＄かんじ$"=> Ok((token::test_helper::new_test_result_span(20, 1, ""),
    Token::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(10, 1, "かんじ"),default_ctx()),
    }));"wide_start")]
    #[test_case("|$hoge$"=> Err(new_error(token::test_helper::new_test_result_span(1, 1, "$hoge$"),nom::error::ErrorKind::TakeWhile1)))]
    fn directive_annotation_works(input: &str) -> IResult {
        default_ctx().directive_annotation(token::Span::new(input))
    }
}
