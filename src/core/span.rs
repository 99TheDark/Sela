use std::{fmt, ops};

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

    pub fn shrink_uniform(&self, radius: u32) -> Self {
        Self { start: self.start + radius, end: self.end - radius }
    }

    pub fn expand(&self, left: u32, right: u32) -> Self {
        Self { start: self.start - left, end: self.end + right }
    }

    pub fn expand_uniform(&self, radius: u32) -> Self {
        Self { start: self.start - radius, end: self.end + radius }
    }

    pub fn debug_src(self, src: &str) -> String {
        src[self].replace('\n', "\\n")
    }

    pub fn src<'a>(self, src: &'a str) -> &'a str {
        &src[self]
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

impl ops::Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start as usize..index.end as usize]
    }
}
