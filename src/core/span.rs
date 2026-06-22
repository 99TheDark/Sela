use std::{fmt, ops::Range};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub const ZERO: Self = Self { start: 0, end: 0 };

    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub const fn single(loc: u32) -> Self {
        Self { start: loc, end: loc }
    }

    pub fn shrink(&self, left: u32, right: u32) -> Self {
        Self { start: self.start + left, end: self.end - right }
    }

    fn range(&self) -> Range<usize> {
        self.start as usize..self.end as usize
    }

    pub fn debug_src(&self, src: &str) -> String {
        src[self.range()].replace('\n', "\\n")
    }

    pub fn src<'a>(&self, src: &'a str) -> &'a str {
        &src[self.range()]
    }

    // Assumes `to` is after `self`
    pub fn to(&self, to: Self) -> Self {
        Self::new(self.start, to.end)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
