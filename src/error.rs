pub mod utils;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    ast,
    error::utils::{Location, NumDigits},
    token::span::Span,
};

pub const CONSOLE_WIDTH: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
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
            line_starts: std::iter::once(0)
                .chain(src.match_indices('\n').map(|m| m.0 + 1))
                .collect(),
            with_color: true,
            errors: Vec::new(),
        }
    }

    pub fn emit(&mut self, message: String, span: Span) {
        self.errors.push(Error { message, span });
    }

    pub fn fail(&mut self, message: String, span: Span) -> ast::Node {
        self.emit(message, span);
        ast::Node::failed(span)
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
        self.src[row..idx as usize].graphemes(true).count()
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

    pub fn print(self) {
        println!("{:?}", self.line_starts);

        // TODO: Currently assumes the error occurs on only one line, the span is not malformed, and the error message is short enough.
        for error in &self.errors {
            let start = self.visual_loc(error.span.start);
            let end = self.visual_loc(error.span.end);

            println!("{:?}: {:?} - {:?}", error.span, start, end);

            println!(
                "{}: {}:{}:{}",
                self.red("Error".to_string()),
                self.file_name,
                start.row,
                start.col
            );

            let left_pad = self.above(start.row);

            let len = if end.col > start.col {
                end.col - start.col
            } else {
                1
            };
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
