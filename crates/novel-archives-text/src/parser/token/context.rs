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
    pub fn term<'b>(&self, input: Span<'b>) -> IResult<'b> {
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
}
