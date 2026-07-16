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

    pub fn src<'a>(self, src: &'a str) -> &'a str {
        &src[self]
    }

    pub fn byte_src<'a>(&self, src: &'a str) -> &'a [u8] {
        &src.as_bytes()[*self]
    }

    pub fn debug_src(self, src: &str) -> String {
        src[self].replace('\n', "\\n")
    }

    // Assumes `to` is after `self`
    // TODO: Maybe change to range for syntax sugar?
    pub fn to(&self, to: Self) -> Self {
        Self::new(self.start, to.end)
    }

    // Assumes u32 is within the span absolutely
    pub fn split(&self, pos: u32) -> (Self, Self) {
        (Self::new(self.start, pos), Self::new(pos + 1, self.end))
    }

    // Assumes u32 is within the span relatively
    pub fn split_relative(&self, pos: u32) -> (Self, Self) {
        (Self::new(self.start, self.start + pos), Self::new(self.start + pos + 1, self.end))
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

impl ops::Index<Span> for [u8] {
    type Output = [u8];

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start as usize..index.end as usize]
    }
}
