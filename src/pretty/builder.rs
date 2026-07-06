use std::io;

use smallvec::SmallVec;

use crate::pretty::{self, Pretty};

pub struct Builder<'a, B: io::Write>(SmallVec<[pretty::Node<'a, B>; 3]>);

impl<'a, B: io::Write> Builder<'a, B> {
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    pub fn empty() -> pretty::ChildNodes<'a, B> {
        SmallVec::new()
    }

    pub fn named(mut self, name: &'a str, inner: &'a dyn Pretty<'a, B>) -> Self {
        self.0.push(pretty::Node::named(name, inner));
        self
    }

    pub fn unnamed(mut self, inner: &'a dyn Pretty<'a, B>) -> Self {
        self.0.push(pretty::Node::unnamed(inner));
        self
    }

    pub fn fold<T, I: IntoIterator<Item = T>, F: Fn(Self, &T) -> Self>(
        self,
        iter: I,
        folder: F,
    ) -> Self {
        let mut builder = self;
        for elem in iter {
            builder = folder(builder, &elem);
        }
        builder
    }

    pub fn finish(self) -> pretty::ChildNodes<'a, B> {
        self.0
    }
}
