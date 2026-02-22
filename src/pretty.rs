use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use smallvec::SmallVec;

use crate::pretty::{
    color::AnsiColor,
    theme::{Coloring, Spacing, Theme},
};

pub mod ast;
pub mod color;
pub mod theme;

/// Automatically generates a SmallVec while casting all items to &dyn Pretty
///
/// # Example
/// ```
/// prettyvec![lhs, op, rhs];
/// ```
#[macro_export]
macro_rules! prettyvec {
    [$(($name:literal, $child:expr)),* $(,)?] => {
        {
            smallvec::smallvec![
                $(
                    crate::pretty::PrettyChild::Named(
                        crate::pretty::NamedPrettyChild::new(
                            $name,
                            $child as &dyn crate::pretty::Pretty,
                        ),
                    ),
                )*
            ]
        }
    };
}

pub enum PrettyChild<'a> {
    Named(NamedPrettyChild<'a>),
    Unnamed(&'a dyn Pretty),
}

pub struct NamedPrettyChild<'a> {
    name: &'a str,
    child: &'a dyn Pretty,
}

impl<'a> NamedPrettyChild<'a> {
    pub fn new(name: &'a str, child: &'a dyn Pretty) -> Self {
        Self { name, child }
    }
}

pub trait Pretty {
    fn title(&self) -> String;

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
        prettyvec![]
    }
}

pub struct Formatter<B: io::Write> {
    buffer: B,
    stack: Vec<bool>,
    theme: Theme,
}

impl<B: io::Write> Formatter<B> {
    pub const fn new(buffer: B, theme: Theme) -> Self {
        Self {
            buffer,
            stack: Vec::new(),
            theme,
        }
    }

    pub fn format(&mut self, node: &dyn Pretty) -> io::Result<()> {
        self.format_node(node, None, false)
    }

    fn format_node(
        &mut self,
        node: &dyn Pretty,
        prefix: Option<&str>,
        is_last: bool,
    ) -> io::Result<()> {
        self.start_line(is_last)?;

        let title = if let Some(color) = node.color() {
            self.maybe_color(node.title(), color)
        } else {
            node.title()
        };

        if let Some(pre) = prefix {
            writeln!(self.buffer, "{}: {}", pre, title)?;
        } else {
            writeln!(self.buffer, "{}", title)?;
        }

        let children = node.children();

        self.stack.push(is_last);
        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            match child {
                PrettyChild::Named(NamedPrettyChild {
                    name,
                    child: pretty,
                }) => self.format_node(*pretty, Some(name), is_last),
                PrettyChild::Unnamed(pretty) => self.format_node(*pretty, None, is_last),
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
        let join = if is_last {
            self.theme.dl_bend
        } else {
            self.theme.l_conn
        };

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

pub fn write_file(file_name: String, src: &dyn Pretty) -> io::Result<()> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);

    Formatter::new(&mut writer, Theme::sharp(Coloring::None, Spacing::Full))
        .format(src)?;

    writer.flush()
}

pub fn print(src: &dyn Pretty) -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut buffer = stdout.lock();

    Formatter::new(&mut buffer, Theme::round(Coloring::Colorful, Spacing::Full))
        .format(src)?;

    buffer.flush()
}
