use crate::error::Diagnostics;

pub trait NumDigits {
    fn num_digits(self) -> u32;
}

impl NumDigits for usize {
    fn num_digits(self) -> u32 {
        self.checked_ilog10().unwrap_or(0) + 1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl<'a> Diagnostics<'a> {
    /*pub(super) fn yellow(&self, s: String) -> String {
        if self.with_color {
            format!("\x1b[33m{}\x1b[0m", s)
        } else {
            s
        }
    }*/

    pub(super) fn red(&self, s: String) -> String {
        if self.with_color { format!("\x1b[31m{}\x1b[0m", s) } else { s }
    }
}
