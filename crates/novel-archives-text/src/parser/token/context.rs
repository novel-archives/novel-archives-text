use super::*;
use nom::InputTake;

#[derive(Debug, PartialEq, Clone, new)]
pub struct Context {
    term_map: TermMap,
}

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct TermKv {
    key: String,
    term: term::Term,
}

impl Context {
    pub fn term<'a, 'b>(&'a self, input: Span<'b>) -> IResult<'b, 'a> {
        let fragment = input.fragment();
        let term_map = &self.term_map.0;
        let first = fragment
            .chars()
            .next()
            .ok_or_else(|| new_error(input, nom::error::ErrorKind::TakeTill1))?;
        let (_, terms) = term_map
            .get(
                term_map
                    .binary_search_by_key(&first, |&(k, _)| k)
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
                term: term.term(),
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
            .map(|t| TermKv::new(t.body().to_string(), t.clone()))
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
        term_map
    }
}
