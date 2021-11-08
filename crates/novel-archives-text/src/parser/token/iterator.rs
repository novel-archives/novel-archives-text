use super::*;

#[derive(new, Debug, PartialEq)]
pub struct RubyIterator<'a> {
    ruby: Span<'a>,
    context: Context,
}

impl<'a> Iterator for RubyIterator<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[derive(new, Debug, PartialEq)]
pub struct RubyBodyIterator<'a> {
    body: Span<'a>,
    context: Context,
}

impl<'a> Iterator for RubyBodyIterator<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[derive(new, Debug, PartialEq)]
pub struct AnnotationBodyIterator<'a> {
    body: Span<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationBodyIterator<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[derive(new, Debug, PartialEq)]
pub struct AnnotationDescriptionIterator<'a> {
    description: Span<'a>,
    context: Context,
}

impl<'a> Iterator for AnnotationDescriptionIterator<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}
