use std::io;

use crate::pretty::{
    self, Pretty,
    color::AnsiColor,
    theme::{Coloring, Spacing, Theme},
};

pub struct Formatter<'w, B: io::Write> {
    buffer: &'w mut B,
    stack: Vec<bool>,
    theme: Theme,
}

impl<'a, 'b, 'w, B: io::Write> Formatter<'w, B>
where
    'a: 'b,
{
    pub const fn new(buffer: &'w mut B, theme: Theme) -> Self {
        Self { buffer, stack: Vec::new(), theme }
    }

    pub fn format(&mut self, node: &'a dyn Pretty<'a, B>) -> pretty::Result {
        self.format_node(node, None, false)
    }

    #[inline(always)]
    pub fn write(&mut self, contents: impl AsRef<str>) -> pretty::Result {
        self.buffer.write_all(contents.as_ref().as_bytes())?;
        Ok(())
    }

    #[inline(always)]
    pub fn write_nl(&mut self) -> pretty::Result {
        self.buffer.write_all(b"\n")?;
        Ok(())
    }

    #[inline(always)]
    pub fn writeln(&mut self, contents: impl AsRef<str>) -> pretty::Result {
        self.write(contents)?;
        self.write_nl()?;
        Ok(())
    }

    fn format_node(
        &mut self,
        node: &'a dyn Pretty<'a, B>,
        name: Option<&str>,
        is_last: bool,
    ) -> pretty::Result {
        self.start_line(is_last)?;

        if let Some(name) = name {
            write!(self.buffer, "{}: ", name)?;
        }

        if let Some(color) = node.color()
            && self.theme.coloring == Coloring::Colorful
        {
            self.write(color.start())?;
            node.fmt_title(self)?;
            self.writeln(color.end())?;
        } else {
            node.fmt_title(self)?;
            self.write_nl()?;
        }

        let children = node.children();

        self.stack.push(is_last);
        for (i, node) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            self.format_node(node.inner, node.name, is_last)?;
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

impl<'w, B: io::Write> io::Write for Formatter<'w, B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}
