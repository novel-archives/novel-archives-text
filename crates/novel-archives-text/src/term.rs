use super::*;

#[derive(Debug, PartialEq, Clone, new)]
pub struct Term {
    id: Id<Term>,
    body: Vec<TokenKind>,
    ruby: Vec<TokenKind>,
    description: Vec<TokenKind>,
}
