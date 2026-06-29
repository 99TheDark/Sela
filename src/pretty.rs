use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use smallvec::{SmallVec, smallvec};

use crate::pretty::{
    color::AnsiColor,
    theme::{Coloring, Spacing, Theme},
};

pub mod ast;
pub mod color;
pub mod theme;

pub struct Builder<'a>(SmallVec<[PrettyChild<'a>; 3]>);

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    pub fn empty() -> PrettyChildren<'a> {
        smallvec![]
    }

    pub fn named(mut self, name: &'a str, inner: &'a dyn Pretty<'a>) -> Self {
        self.0.push(PrettyChild::Named { name, inner });
        self
    }

    pub fn prefixed(
        mut self,
        name: &'a str,
        prefix: &'a str,
        inner: &'a dyn Pretty<'a>,
    ) -> Self {
        self.0.push(PrettyChild::Prefixed { name, prefix, inner });
        self
    }

    pub fn unnamed(mut self, inner: &'a dyn Pretty<'a>) -> Self {
        self.0.push(PrettyChild::Unnamed { inner });
        self
    }

    pub fn finish(self) -> PrettyChildren<'a> {
        self.0
    }
}

pub enum PrettyChild<'a> {
    Named { name: &'a str, inner: &'a dyn Pretty<'a> },
    Prefixed { name: &'a str, prefix: &'a str, inner: &'a dyn Pretty<'a> },
    Unnamed { inner: &'a dyn Pretty<'a> },
}

pub type PrettyChildren<'a> = SmallVec<[PrettyChild<'a>; 3]>;

pub trait Pretty<'a> {
    fn title(&self) -> String;

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&'a self) -> PrettyChildren<'a>;
}

pub struct Formatter<B: io::Write> {
    buffer: B,
    stack: Vec<bool>,
    theme: Theme,
}

impl<'a, B: io::Write> Formatter<B> {
    pub const fn new(buffer: B, theme: Theme) -> Self {
        Self { buffer, stack: Vec::new(), theme }
    }

    pub fn format(&mut self, node: &'a dyn Pretty<'a>) -> io::Result<()> {
        self.format_node(node, None, None, false)
    }

    fn format_node(
        &mut self,
        node: &'a dyn Pretty<'a>,
        name: Option<&str>,
        prefix: Option<&str>,
        is_last: bool,
    ) -> io::Result<()> {
        self.start_line(is_last)?;

        let title = if let Some(color) = node.color() {
            self.maybe_color(node.title(), color)
        } else {
            node.title()
        };

        let colored = if self.theme.coloring == Coloring::Colorful
            && let Some(color) = node.color()
        {
            write!(self.buffer, "{}", color.color_start())?;
            true
        } else {
            false
        };

        match (name, prefix) {
            (Some(name), Some(prefix)) => {
                write!(self.buffer, "{}: {} {}", name, prefix, title)
            }
            (Some(name), None) => write!(self.buffer, "{}: {}", name, title),
            (None, Some(prefix)) => write!(self.buffer, "{} {}", prefix, title),
            (None, None) => write!(self.buffer, "{}", title),
        }?;

        if colored {
            writeln!(self.buffer, "{}", AnsiColor::color_end())?;
        } else {
            writeln!(self.buffer)?;
        }

        let children = node.children();

        self.stack.push(is_last);
        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            match child {
                PrettyChild::Named { name, inner } => {
                    self.format_node(*inner, Some(name), None, is_last)
                }
                PrettyChild::Prefixed { name, prefix, inner } => {
                    self.format_node(*inner, Some(name), Some(prefix), is_last)
                }
                PrettyChild::Unnamed { inner } => {
                    self.format_node(*inner, None, None, is_last)
                }
            }?;
        }
        self.stack.pop();

        Ok(())
    }

    fn start_line(&mut self, is_last: bool) -> std::io::Result<()> {
        if self.stack.is_empty() {
            return Ok(());
        }

        for &parent_last in &self.stack[1..] {
            if parent_last {
                self.buffer.write_all(self.empty())?;
            } else {
                self.buffer.write_all(self.down().as_bytes())?;
            }
        }

        self.buffer.write_all(self.connector(is_last).as_bytes())
    }

    fn maybe_color(&self, s: String, color: AnsiColor) -> String {
        match self.theme.coloring {
            Coloring::Colorful => color.color(s),
            Coloring::None => s,
        }
    }

    fn empty(&self) -> &'static [u8] {
        match self.theme.spacing {
            Spacing::Full => b"   ",
            Spacing::Compact => b"  ",
        }
    }

    fn down(&self) -> String {
        match (self.theme.coloring, self.theme.spacing) {
            (Coloring::Colorful, Spacing::Full) => {
                format!("{}  ", AnsiColor::Gray.color(self.theme.v_bar))
            }
            (Coloring::Colorful, Spacing::Compact) => {
                format!("{} ", AnsiColor::Gray.color(self.theme.v_bar))
            }
            (Coloring::None, Spacing::Full) => format!("{}  ", self.theme.v_bar),
            (Coloring::None, Spacing::Compact) => format!("{} ", self.theme.v_bar),
        }
    }

    fn connector(&self, is_last: bool) -> String {
        let join = if is_last { self.theme.dl_bend } else { self.theme.l_conn };

        match (self.theme.coloring, self.theme.spacing) {
            (Coloring::Colorful, Spacing::Full) => {
                format!(
                    "{}{} ",
                    AnsiColor::Gray.color(join),
                    AnsiColor::Gray.color(self.theme.h_bar)
                )
            }
            (Coloring::Colorful, Spacing::Compact) => {
                format!("{} ", AnsiColor::Gray.color(join))
            }
            (Coloring::None, Spacing::Full) => format!("{}{} ", join, self.theme.h_bar),
            (Coloring::None, Spacing::Compact) => format!("{} ", join),
        }
    }
}

impl<B: io::Write> io::Write for Formatter<B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

pub fn write_file<'a>(file_name: String, src: &'a dyn Pretty<'a>) -> io::Result<()> {
    let file = File::create(file_name)?;
    let mut buffer = BufWriter::new(file);

    Formatter::new(&mut buffer, Theme::sharp(Coloring::None, Spacing::Full))
        .format(src)?;

    buffer.flush()
}

pub fn print<'a>(src: &'a dyn Pretty<'a>) -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut buffer = stdout.lock();

    Formatter::new(&mut buffer, Theme::round(Coloring::Colorful, Spacing::Full))
        .format(src)?;

    buffer.flush()
}
