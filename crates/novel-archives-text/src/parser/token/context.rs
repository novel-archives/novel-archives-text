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
    pub const MAX_RUBY_COUNT_PER_BODY_CHAR: usize = 10;
    pub const MAX_RUBY_COUNT_BODY: usize = 10;
    pub fn term<'a>(&self, input: ParsedSpan<'a>) -> IResult<'a> {
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
            ParsedToken::Term {
                body,
                term: term.term().clone(),
            },
        ))
    }

    pub fn directive_ruby<'a>(&self, input: ParsedSpan<'a>) -> IResult<'a> {
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
            let max_ruby_count = body_count * Self::MAX_RUBY_COUNT_PER_BODY_CHAR;

            if without_variation_selector_count(ruby.fragment()) <= max_ruby_count {
                Ok((
                    after_parsed_ruby,
                    ParsedToken::Ruby {
                        body: iterator::RubyBodyIterator::new(body, self.clone()),
                        ruby: iterator::RubyIterator::new(ruby),
                    },
                ))
            } else {
                Ok((after_parsed_directive, ParsedToken::Other(directive)))
            }
        }
    }

    pub fn directive_annotation<'a>(&self, input: ParsedSpan<'a>) -> IResult<'a> {
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
                ParsedToken::Annotation {
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
            TokenText::new(vec![Token::new_kanji(Span::new(
                text.into(),
                Position::new(0, 0),
            ))]),
            TokenText::new(vec![Token::new_hiragana(Span::new(
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
        ParsedToken::Term{
            body:token::test_helper::new_test_result_span(0, 1, "無"),
            term:Arc::new(new_sample_term("無")),
        }))
    )]
    #[test_case(TermMap::new(&[
            new_sample_term("穂積"),
    ]),"穂積しょう"
        =>Ok((token::test_helper::new_test_result_span(6, 1, "しょう"),
        ParsedToken::Term{
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
        ParsedToken::Term{
            body:token::test_helper::new_test_result_span(0, 1, "穂積しょう"),
            term:Arc::new(new_sample_term("穂積しょう")),
        }))
    )]
    fn context_term_works(term_map: TermMap, input: &str) -> IResult {
        let ctx = Context::new(Arc::new(term_map));
        ctx.term(token::ParsedSpan::new(input))
    }

    fn default_ctx() -> Context {
        Context::new(Arc::new(TermMap::new(&[])))
    }

    #[test_case("|漢字(かんじ)"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    ParsedToken::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字"),default_ctx()),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ")),
    })))]
    #[test_case("|ほげ（ふが)"=> Ok((token::test_helper::new_test_result_span(17, 1, ""),
    ParsedToken::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "ほげ"),default_ctx()),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(10, 1, "ふが")),
    })))]
    #[test_case("|ふ符(hoho）"=> Ok((token::test_helper::new_test_result_span(15, 1, ""),
    ParsedToken::Ruby{
        body: iterator::RubyBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "ふ符"),default_ctx()),
        ruby: iterator::RubyIterator::new(token::test_helper::new_test_result_span(8, 1, "hoho")),
    })))]
    #[test_case("|(かんじ)"=> Ok((token::test_helper::new_test_result_span(1, 1, "(かんじ)"),ParsedToken::Ignore(token::test_helper::new_test_result_span(0, 1, "|"))));"half_directive")]
    #[test_case("｜(かんじ)"=> Ok((token::test_helper::new_test_result_span(3, 1, "(かんじ)"),ParsedToken::Ignore(token::test_helper::new_test_result_span(0, 1, "｜"))));"wide_directive")]
    fn directive_ruby_works(input: &str) -> IResult {
        default_ctx().directive_ruby(token::ParsedSpan::new(input))
    }

    #[test_case("|漢字$かんじ$"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    ParsedToken::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ"),default_ctx()),
    }));"half_all")]
    #[test_case("|漢字(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(32, 1, ""),
    ParsedToken::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字(かんじ)"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(19, 1, "せつめい"),default_ctx()),
    }));"with_ruby")]
    #[test_case("||漢字ふ(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(36, 1, ""),
    ParsedToken::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "|漢字ふ(かんじ)"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(23, 1, "せつめい"),default_ctx()),
    }));"with_ruby_directive")]
    #[test_case("|漢字＄かんじ$"=> Ok((token::test_helper::new_test_result_span(20, 1, ""),
    ParsedToken::Annotation{
        body: iterator::AnnotationBodyIterator::new(token::test_helper::new_test_result_span(1, 1, "漢字"),default_ctx()),
        description: iterator::AnnotationDescriptionIterator::new(token::test_helper::new_test_result_span(10, 1, "かんじ"),default_ctx()),
    }));"wide_start")]
    #[test_case("|$hoge$"=> Err(new_error(token::test_helper::new_test_result_span(1, 1, "$hoge$"),nom::error::ErrorKind::TakeWhile1)))]
    fn directive_annotation_works(input: &str) -> IResult {
        default_ctx().directive_annotation(token::ParsedSpan::new(input))
    }
}
