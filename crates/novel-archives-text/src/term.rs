use super::*;

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct Term {
    id: Id<Term>,
    body: TokenText,
    ruby: TokenText,
    description: TokenText,
}
