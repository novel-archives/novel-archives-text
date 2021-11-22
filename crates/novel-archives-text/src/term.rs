use super::*;

#[derive(Debug, PartialEq, Clone, new, Getters)]
pub struct Term {
    id: Id<Term>,
    body: String,
    ruby: String,
    description: TokenText,
}
