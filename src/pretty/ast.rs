use std::io;

use bumpalo::collections::Vec as BumpVec;

use crate::{
    ast::{self, NodeRef, symbol::Symbolic},
    pretty::{self, AnsiColor, Pretty},
};

impl<'a, B: io::Write> Pretty<'a, B> for ast::Node<'a> {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        use crate::ast::NodeKind::*;
        match &self.kind {
            Ident(name) => f.write(format!("Identifier ({})", name)),
            BinOp { .. } => f.write("Binary Operation"),
            KwBinOp { .. } => f.write("Binary Keyword Operation"),
            Comp { .. } => f.write("Comparison"),
            Range { .. } => f.write("Range"),
            UnOp { .. } => f.write("Unary Operation"),
            Access { .. } => f.write("Member Access"),
            Invoc { .. } => f.write("Invocation"),
            Int(val) => f.write(format!("Integer ({})", val)),
            Float(val) => f.write(format!("Floating-Point Number ({})", val)),
            Bool(val) => f.write(format!("Boolean ({})", val)),
            Char(val) => f.write(format!("Character ({})", val.escape_debug())),
            String(_) => f.write("String"),
            Decl { .. } => f.write("Declaration"),
            Assign { .. } => f.write("Assignment"),
            If { .. } => f.write("If Statement"),
            Loop { .. } => f.write("Loop"),
            While { .. } => f.write("While Loop"),
            For { .. } => f.write("For Loop"),
            FuncSig { .. } => f.write("Function Signature"),
            Func { .. } => f.write("Function"),
            Block(e) if e.is_empty() => f.write("Empty Block"),
            Parens(_) => f.write("Parentheses"),
            Block(_) => f.write("Block"),
            Pair { .. } => f.write("Pair"),
            Use { .. } => f.write("Use"),
            Charm => f.write("Charm"),
            Unknown => f.write("<! Unknown !>"),
            UnknownInt => f.write("<! Unknown Integer !>"),
            UnknownFloat => f.write("<! Unknown Floating-Point Number !>"),
            UnknownRange { .. } => f.write("<! Unknown Range !>"),
        }
    }

    fn color(&self) -> Option<AnsiColor> {
        use ast::NodeKind::*;
        let col = match &self.kind {
            // BinOp(..) | Comp(..) | Range(..) | UnOp(..) => AnsiColor::White,
            Use { .. } | Charm => AnsiColor::Blue,
            KwBinOp { .. } | If { .. } | Loop { .. } | While { .. } | For { .. } => {
                AnsiColor::Purple
            }
            Access { .. } => AnsiColor::Yellow,
            Ident(_) => AnsiColor::Cyan,
            Decl { .. } | Assign { .. } => AnsiColor::Green,
            Block(e) if e.is_empty() => AnsiColor::Gray,
            Unknown => AnsiColor::Red,
            _ => return None,
        };
        Some(col)
    }

    fn children(&'a self) -> pretty::ChildNodes<'a, B> {
        use ast::NodeKind::*;
        match &self.kind {
            BinOp { lhs, op, rhs } => pretty::Builder::new()
                .named("Left-hand Side", *lhs)
                .named("Operator", op)
                .named("Right-hand Side", *rhs)
                .finish(),
            KwBinOp { lhs, op, rhs } => pretty::Builder::new()
                .named("Left-hand Side", *lhs)
                .named("Operator", op)
                .named("Right-hand Side", *rhs)
                .finish(),
            Comp { lhs, comp, rhs } => pretty::Builder::new()
                .named("Left-hand Side", *lhs)
                .named("Comparator", comp)
                .named("Right-hand Side", *rhs)
                .finish(),
            Range { from, range, to } | UnknownRange { from, range, to } => {
                pretty::Builder::new()
                    .named("From", from)
                    .named("Range Type", range)
                    .named("To", to)
                    .finish()
            }
            UnOp { op, rhs } => pretty::Builder::new()
                .named("Operator", op)
                .named("Right-hand Side", *rhs)
                .finish(),
            Access { parent, child } => pretty::Builder::new()
                .named("Parent", *parent)
                .named("Child", *child)
                .finish(),
            Invoc { callee, args } => pretty::Builder::new()
                .named("Callee", *callee)
                .named("Arguments", *args)
                .finish(),
            String(frags) => pretty::Builder::new()
                .fold(*frags, |builder, frag| match frag {
                    ast::string::Fragment::String(inner) => {
                        builder.named("Raw String", inner)
                    }
                    ast::string::Fragment::Expr(inner) => {
                        builder.named("Interpolated Expression", *inner)
                    }
                })
                .finish(),
            Decl { pat, val } => pretty::Builder::new()
                .named("Pattern", *pat)
                .named("Value", *val)
                .finish(),
            Assign { pat, assign, val } => pretty::Builder::new()
                .named("Pattern", *pat)
                .named("Assigner", assign)
                .named("Value", *val)
                .finish(),
            If { cond, body, fallback } => pretty::Builder::new()
                .named("Condition", *cond)
                .named("Then Body", *body)
                .named("Else Body", fallback)
                .finish(),
            Loop { body } => pretty::Builder::new().named("Body", *body).finish(),
            While { cond, body } => pretty::Builder::new()
                .named("Condition", *cond)
                .named("Body", *body)
                .finish(),
            For { vari, iter, body } => pretty::Builder::new()
                .named("Variable", *vari)
                .named("Iterable", *iter)
                .named("Body", *body)
                .finish(),
            FuncSig { params, ret } => pretty::Builder::new()
                .named("Parameters", *params)
                .named("Return Type", ret)
                .finish(),
            Func { name, sig, body } => pretty::Builder::new()
                .named("Name", name)
                .named("Signature", *sig)
                .named("Body", body)
                .finish(),
            Use { path } => pretty::Builder::new().named("Path", *path).finish(),
            Parens(elems) | Block(elems) => elems.children(),
            Pair { lhs, rhs } => {
                pretty::Builder::new().named("Left", *lhs).named("Right", *rhs).finish()
            }
            Ident(_) | Int(_) | Float(_) | Bool(_) | Char(_) | Charm | Unknown
            | UnknownInt | UnknownFloat => pretty::Builder::empty(),
        }
    }
}

// TODO: Move this stuff to other files
impl<'a, B: io::Write, T: Symbolic> Pretty<'a, B> for T {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        f.write(format!("{} ({})", self.name(), self.as_str()))
    }

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&self) -> pretty::ChildNodes<'a, B> {
        pretty::Builder::empty()
    }
}

impl<'a, B: io::Write> Pretty<'a, B> for Vec<NodeRef<'a>> {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        f.write(if self.is_empty() { "Empty List" } else { "List" })
    }

    fn color(&self) -> Option<AnsiColor> {
        if self.is_empty() { Some(AnsiColor::Gray) } else { None }
    }

    fn children(&self) -> pretty::ChildNodes<'a, B> {
        self.iter().map(|node| pretty::Node::unnamed(*node)).collect()
    }
}

impl<'a, B: io::Write> Pretty<'a, B> for BumpVec<'a, NodeRef<'a>> {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        f.write(if self.is_empty() { "Empty List" } else { "List" })
    }

    fn color(&self) -> Option<AnsiColor> {
        if self.is_empty() { Some(AnsiColor::Gray) } else { None }
    }

    fn children(&self) -> pretty::ChildNodes<'a, B> {
        self.iter().map(|node| pretty::Node::unnamed(*node)).collect()
    }
}

impl<'a, B: io::Write> Pretty<'a, B> for &'a [ast::NodeRef<'a>] {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        f.write(if self.is_empty() { "Empty List" } else { "List" })
    }

    fn color(&self) -> Option<AnsiColor> {
        if self.is_empty() { Some(AnsiColor::Gray) } else { None }
    }

    fn children(&self) -> pretty::ChildNodes<'a, B> {
        self.iter().map(|node| pretty::Node::unnamed(*node)).collect()
    }
}

impl<'a, B: io::Write> Pretty<'a, B> for Option<NodeRef<'a>> {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        match self {
            Some(inner) => {
                f.write("Some ")?;
                inner.fmt_title(f)
            }
            None => f.write("None"),
        }
    }

    fn color(&self) -> Option<AnsiColor> {
        if self.is_some() { None } else { Some(AnsiColor::Gray) }
    }

    fn children(&self) -> pretty::ChildNodes<'a, B> {
        self.map_or_else(|| pretty::Builder::empty(), |inner| inner.children())
    }
}

impl<'a, B: io::Write> Pretty<'a, B> for String {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result {
        f.write(format!("\"{}\"", self.escape_debug()))
    }

    fn children(&'a self) -> pretty::ChildNodes<'a, B> {
        pretty::Builder::empty()
    }
}
