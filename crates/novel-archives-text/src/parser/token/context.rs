use super::*;
#[derive(Default, Debug, PartialEq, Clone, new)]
pub struct Context<LF: Fn(Span) -> IResult, GF: Fn(Span) -> IResult> {
    local_term_complete: LF,
    global_term_complete: GF,
}

pub trait CompleteTypes {
    type Local: Fn(Span) -> IResult;
    type Global: Fn(Span) -> IResult;
}
