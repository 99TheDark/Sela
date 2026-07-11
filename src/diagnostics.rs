pub mod natural;
pub mod utils;

use bumpalo::Bump;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    ast,
    core::span::Span,
    diagnostics::utils::{Location, NumDigits},
};

pub const CONSOLE_WIDTH: usize = 80;

pub enum Severity {
    Error,
    Warning,
    // Hint,
    // Note,
}

// TODO: Make error just have ErrorKind and Span(s) and make this use trait impls and variant data to store info for printing
// Maybe subspans can be held in the structure? Also maybe use span arena and point some larger errors to other definitions
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match self {
            Syntax => "Syntax",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostics<'a> {
    file_name: String,
    src: &'a str,
    line_starts: Vec<usize>,
    with_color: bool,
    errors: Vec<Error>,
}

impl<'a> Diagnostics<'a> {
    pub fn new(file_name: String, src: &'a str) -> Self {
        Self {
            file_name,
            src,
            // ! This is a bottleneck
            line_starts: std::iter::once(0)
                .chain(src.match_indices('\n').map(|m| m.0 + 1))
                .collect(),
            with_color: true,
            errors: Vec::new(),
        }
    }

    #[inline]
    #[cold]
    pub fn emit(&mut self, kind: ErrorKind, message: String, span: Span) {
        self.errors.push(Error { kind, message, span });
    }

    #[cold]
    pub fn fail<'ast>(
        &mut self,
        kind: ErrorKind,
        message: String,
        span: Span,
        alloc: &'ast Bump,
    ) -> &'ast ast::Node<'ast> {
        self.emit(kind, message, span);
        alloc.alloc(ast::Node::failed(span))
    }

    fn nth_line(&self, line_num: usize) -> &str {
        if line_num < self.line_starts.len() - 1 {
            &self.src[self.line_starts[line_num]..self.line_starts[line_num + 1] - 1]
        } else {
            &self.src[self.line_starts[line_num]..]
        }
    }

    fn row(&self, idx: u32) -> usize {
        let line_start_idx = self.line_starts.partition_point(|x| *x <= idx as usize);
        line_start_idx - 1
    }

    fn column(&self, row: usize, idx: u32) -> usize {
        self.src[self.line_starts[row]..idx as usize].graphemes(true).count()
    }

    fn visual_loc(&self, idx: u32) -> Location {
        let row = self.row(idx);
        let col = self.column(row, idx);
        Location::new(row, col)
    }

    fn above(&self, row: usize) -> u32 {
        const LEADING_SIZE: usize = 4;
        let top_row = row.saturating_sub(LEADING_SIZE);
        let left_pad = (row + 1).num_digits();

        for i in top_row..=row {
            let n = i + 1;
            let pad = left_pad - n.num_digits();
            println!("{}{} | {}", n, " ".repeat(pad as usize), self.nth_line(i));
        }

        left_pad
    }

    #[inline]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn print(self) {
        // TODO: Currently assumes the error occurs on only one line, the span is not malformed, and the error message is short enough.
        for error in &self.errors {
            let start = self.visual_loc(error.span.start);
            let end = self.visual_loc(error.span.end);

            println!(
                "{}: {}:{}:{}",
                self.red(format!("{} Error", error.kind)),
                self.file_name,
                start.row + 1,
                start.col + 1
            );

            let left_pad = self.above(start.row);

            let len = if end.col > start.col { end.col - start.col } else { 1 };
            println!(
                "{} | {}{} {}\n",
                " ".repeat(left_pad as usize),
                " ".repeat(start.col),
                self.red("^".repeat(len)),
                self.red(error.message.clone())
            );
        }
    }
}
