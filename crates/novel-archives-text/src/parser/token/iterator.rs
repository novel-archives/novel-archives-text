use super::*;

#[derive(new, Debug, PartialEq)]
pub struct RubyIterator<'a> {
    ruby: Span<'a>,
}

impl<'a> Iterator for RubyIterator<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[derive(new, Debug, PartialEq)]
pub struct RubyBodyIterator<'a> {
    ruby: Span<'a>,
}

impl<'a> Iterator for RubyBodyIterator<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}
