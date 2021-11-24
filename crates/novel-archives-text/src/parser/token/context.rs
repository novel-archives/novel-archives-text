use nom::bytes::complete::{take_while1, take_while_m_n};
use nom::sequence::{delimited, tuple};
use nom_extend::character::complete;
use std::cmp::Reverse;
use std::collections::BTreeMap;

use super::*;

#[derive(Debug, PartialEq, Clone, new)]
pub struct ParseContext {
    term_map: Arc<BTreeMap<String, term::Term>>,
}

impl ParseContext {
    pub fn term<'a>(&self, input: ParsedSpan<'a>) -> IResult<'a> {
        let (input, parsed) = delimited(
            take_while_m_n(1, 1, character::is_start_term),
            complete::able_to_term,
            take_while_m_n(1, 1, character::is_end_term),
        )(input)?;
        let term = self
            .term_map
            .get(*parsed.fragment())
            .ok_or_else(|| new_error(input, nom::error::ErrorKind::TakeTill1))?;
        Ok((
            input,
            ParsedToken::Term {
                body: parsed,
                term_id: term.id().clone(),
            },
        ))
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
                    body,
                    description: iterator::TextIterator::new(description, self.clone()),
                },
            )
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TermMap(Vec<(char, Vec<term::Term>)>);

impl TermMap {
    pub fn new(terms: Vec<term::Term>) -> Self {
        let mut term_map = Self(Vec::with_capacity(terms.len()));

        for term in terms.iter() {
            let ck = term.body().chars().next().unwrap();
            match term_map.0.binary_search_by_key(&ck, |(k, _)| *k) {
                Ok(i) => {
                    let (_, terms) = term_map.0.get_mut(i).unwrap();
                    terms.push(term.clone());
                }
                Err(_) => {
                    term_map.0.push((ck, vec![term.clone()]));
                    term_map.0.sort_by_key(|(k, _)| *k);
                }
            }
        }
        for (_, terms) in term_map.0.iter_mut() {
            terms.sort_by_key(|kv| Reverse(without_variation_selector_count(kv.body())));
        }
        term_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn new_sample_term(id: &str, text: &str) -> term::Term {
        term::Term::new(
            Id::new(id.into()),
            text.into(),
            "む".into(),
            TokenText::new(vec![]),
        )
    }
    #[test_case(vec![
        new_sample_term("term_id1", "無"),
    ],"無"
        =>Err(new_error(token::test_helper::new_test_result_span(0, 1, "無"),nom::error::ErrorKind::TakeWhileMN))
    )]
    #[test_case(vec![
        new_sample_term("term_id1", "無"),
    ],"\"無\""
        => Ok((token::test_helper::new_test_result_span(5, 1, ""),ParsedToken::Term{
        body: token::test_helper::new_test_result_span(1, 1, "無"),
        term_id:Id::new("term_id1".into()),
        }))
    ;"quote無")]
    #[test_case(vec![
            new_sample_term("term_id1","穂積"),
    ],"\"穂積\"しょう"
        => Ok((token::test_helper::new_test_result_span(8, 1, "しょう"),ParsedToken::Term{
        body: token::test_helper::new_test_result_span(1, 1, "穂積"),
        term_id:Id::new("term_id1".into()),
        }))
    )]
    #[test_case(vec![
            new_sample_term("term_id1","穂積"),
    ],"しょう"
        =>Err(new_error(token::test_helper::new_test_result_span(0, 1, "しょう"),nom::error::ErrorKind::TakeWhileMN))
    )]
    #[test_case(vec![
            new_sample_term("term_id1","穂積"),
    ],"穂"
        =>Err(new_error(token::test_helper::new_test_result_span(0, 1, "穂"),nom::error::ErrorKind::TakeWhileMN))
    )]
    #[test_case(vec![
            new_sample_term("term_id1","穂積しょう"),
            new_sample_term("term_id2","穂積"),
    ],"\"穂積しょう\""
        => Ok((token::test_helper::new_test_result_span(17, 1, ""),ParsedToken::Term{
        body: token::test_helper::new_test_result_span(1, 1, "穂積しょう"),
        term_id:Id::new("term_id1".into()),
        }))
    )]
    fn context_term_works(terms: Vec<term::Term>, input: &str) -> IResult {
        let ctx = ParseContext::new(Arc::new(
            terms
                .into_iter()
                .map(|term| (term.body().clone(), term))
                .collect(),
        ));
        ctx.term(token::ParsedSpan::new(input))
    }

    /*
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

        pub fn fire_terms() -> Vec<term::Term> {
            vec![term::Term::new(
                Id::new("term_id1".into()),
                "\"炎\"".into(),
                "".into(),
                TokenText::new(vec![]),
            )]
        }
    }

    #[test_case(token_works_testdata::hit_terms(),"穂積しょう" => Ok((
                token::test_helper::new_test_result_span(15, 1, ""),
                ParsedToken::Term {
                    body: token::test_helper::new_test_result_span(0, 1, "穂積しょう"),
                    term_id: Id::new("term_id1".into()),
                },
            )))]
    #[test_case(token_works_testdata::hit_terms(),"穂積しょうたろう" => Ok((
                token::test_helper::new_test_result_span(15, 1, "たろう"),
                ParsedToken::Term {
                    body: token::test_helper::new_test_result_span(0, 1, "穂積しょう"),
                    term_id: Id::new("term_id1".into()),
                },
            )))]
    #[test_case(token_works_testdata::other_terms(),"穂積しょうたろう" => Ok((
                token::test_helper::new_test_result_span(6, 1, "しょうたろう"),
                ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "穂積")),
            )))]
    #[test_case(token_works_testdata::hit_terms(),"|穂積《ほづみ》しょうたろう" => Ok((
                token::test_helper::new_test_result_span(22, 1, "しょうたろう"),
                ParsedToken::Ruby{
                    body:token::test_helper::new_test_result_span(1, 1, "穂積"),
                    ruby:token::test_helper::new_test_result_span(10, 1, "ほづみ"),
                }
            )))]
    #[test_case(token_works_testdata::hit_terms(),"穂積《ほづみ》しょうたろう" => Ok((
                token::test_helper::new_test_result_span(21, 1, "しょうたろう"),
                ParsedToken::KanjiRuby{
                    body:token::test_helper::new_test_result_span(0, 1, "穂積"),
                    ruby:token::test_helper::new_test_result_span(9, 1, "ほづみ"),
                }
            ));"kanji_ruby1")]
    #[test_case(token_works_testdata::hit_terms(),"穂積(ほづみ)しょうたろう" => Ok((
                token::test_helper::new_test_result_span(17, 1, "しょうたろう"),
                ParsedToken::KanjiRuby{
                    body:token::test_helper::new_test_result_span(0, 1, "穂積"),
                    ruby:token::test_helper::new_test_result_span(7, 1, "ほづみ"),
                }
            ));"kanji_ruby2")]
    #[test_case(token_works_testdata::hit_terms(),"|穂積しょうたろう" => Ok((
                token::test_helper::new_test_result_span(1, 1, "穂積しょうたろう"),
                ParsedToken::Plaintext(token::test_helper::new_test_result_span(0, 1, "|"))
            )))]
    #[test_case(token_works_testdata::fire_terms(),"\"炎\"の穂積しょうたろう" => Ok((
                token::test_helper::new_test_result_span(5, 1, "の穂積しょうたろう"),
                ParsedToken::Term{
                    body:token::test_helper::new_test_result_span(0, 1, "\"炎\""),
                    term_id:Id::new("term_id1".into()),
                }
            )))]
    fn context_token_works(terms: Vec<term::Term>, input: &str) -> IResult {
        let ctx = ParseContext::new(Arc::new(TermMap::new(terms)));
        ctx.token(token::ParsedSpan::new(input))
    }
    */

    fn default_ctx() -> ParseContext {
        ParseContext::new(Arc::new(HashMap::new()))
    }

    #[test_case("|漢字$かんじ$"=> Ok((token::test_helper::new_test_result_span(18, 1, ""),
    ParsedToken::Annotation{
        body: token::test_helper::new_test_result_span(1, 1, "漢字"),
        description: iterator::TextIterator::new(token::test_helper::new_test_result_span(8, 1, "かんじ"),default_ctx()),
    }));"half_all")]
    #[test_case("|漢字(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(32, 1, ""),
    ParsedToken::Annotation{
        body: token::test_helper::new_test_result_span(1, 1, "漢字(かんじ)"),
        description: iterator::TextIterator::new(token::test_helper::new_test_result_span(19, 1, "せつめい"),default_ctx()),
    }));"with_ruby")]
    #[test_case("||漢字ふ(かんじ)$せつめい$"=> Ok((token::test_helper::new_test_result_span(36, 1, ""),
    ParsedToken::Annotation{
        body: token::test_helper::new_test_result_span(1, 1, "|漢字ふ(かんじ)"),
        description: iterator::TextIterator::new(token::test_helper::new_test_result_span(23, 1, "せつめい"),default_ctx()),
    }));"with_ruby_directive")]
    #[test_case("|漢字＄かんじ$"=> Ok((token::test_helper::new_test_result_span(20, 1, ""),
    ParsedToken::Annotation{
        body: token::test_helper::new_test_result_span(1, 1, "漢字"),
        description: iterator::TextIterator::new(token::test_helper::new_test_result_span(10, 1, "かんじ"),default_ctx()),
    }));"wide_start")]
    #[test_case("|$hoge$"=> Err(new_error(token::test_helper::new_test_result_span(1, 1, "$hoge$"),nom::error::ErrorKind::TakeWhile1)))]
    fn directive_annotation_works(input: &str) -> IResult {
        default_ctx().directive_annotation(token::ParsedSpan::new(input))
    }
}
