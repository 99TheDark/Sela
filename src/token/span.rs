use std::{cmp, fmt};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Location {
    pub idx: usize,
    pub row: usize,
    pub col: usize,
}

impl Location {
    pub const ZERO: Self = Self::new(0, 0, 0);

    pub const fn new(idx: usize, row: usize, col: usize) -> Self {
        Self { idx, row, col }
    }

    pub const fn next(self) -> Self {
        Self {
            idx: self.idx + 1,
            ..self
        }
    }
}

impl cmp::Ord for Location {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:?}:{:?})", self.idx, self.row, self.col)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub const ZERO: Self = Self {
        start: Location::ZERO,
        end: Location::ZERO,
    };

    pub const fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub const fn single(loc: Location) -> Self {
        Self {
            start: loc,
            end: loc,
        }
    }

    pub fn debug_src(&self, src: &str) -> String {
        src[self.start.idx..self.end.idx].replace('\n', "\\n")
    }

    pub fn src<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start.idx..self.end.idx]
    }

    pub fn to(&self, to: Self) -> Self {
        Self::new(self.start, to.end)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} to {:?}", self.start, self.end)
    }
}
