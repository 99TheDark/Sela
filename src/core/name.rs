use std::{collections::HashSet, ptr};

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Name<'interner>(&'interner str);

impl<'i> std::cmp::PartialEq for Name<'i> {
    fn eq(&self, other: &Self) -> bool {
        // Guaranteed to be the same length and not share an address
        ptr::eq(self.0.as_ptr(), other.0.as_ptr())
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub struct NameInterner<'a> {
    names: HashSet<Name<'a>>,
}

impl<'a> NameInterner<'a> {
    pub fn new() -> Self {
        Self { names: HashSet::new() }
    }

    pub fn add(&mut self, name: Name<'a>) {
        self.names.insert(name);
    }
}
