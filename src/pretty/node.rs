use std::io;

use smallvec::SmallVec;

use crate::pretty::Pretty;

pub struct Node<'a, B: io::Write> {
    pub name: Option<&'a str>,
    pub inner: &'a dyn Pretty<'a, B>,
}

impl<'a, B: io::Write> Node<'a, B> {
    #[inline(always)]
    pub fn named(name: &'a str, inner: &'a dyn Pretty<'a, B>) -> Self {
        Self { name: Some(name), inner }
    }

    #[inline(always)]
    pub fn unnamed(inner: &'a dyn Pretty<'a, B>) -> Self {
        Self { name: None, inner }
    }
}

pub type ChildNodes<'a, B> = SmallVec<[Node<'a, B>; 3]>;
