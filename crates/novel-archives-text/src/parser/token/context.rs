use super::*;
use nom::InputTake;

#[derive(Debug, PartialEq, Clone, new)]
pub struct Context {
    term_map: TermMap,
}

pub type TermMap = Vec<(char, Vec<TermKv>)>;

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct TermKv {
    key: String,
    term: term::Term,
}

impl Context {
    pub fn term<'a, 'b>(&'a self, input: Span<'b>) -> IResult<'b, 'a> {
        let fragment = input.fragment();
        let term_map = &self.term_map;
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
