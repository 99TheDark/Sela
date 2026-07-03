use crate::{
    ast::{self, NodeRef, symbol::Symbolic},
    pretty::{self, AnsiColor, Pretty, PrettyChild, PrettyChildren},
};

impl<'a> Pretty<'a> for ast::Node<'a> {
    fn title(&self) -> String {
        use ast::NodeKind::*;
        match &self.kind {
            Ident(name) => format!("Identifier ({})", name),
            BinOp { .. } => "Binary Operation".to_string(),
            KwBinOp { .. } => "Binary Keyword Operation".to_string(),
            Comp { .. } => "Comparison".to_string(),
            Range { .. } => "Range".to_string(),
            UnOp { .. } => "Unary Operation".to_string(),
            Access { .. } => "Member Access".to_string(),
            Invoc { .. } => "Invocation".to_string(),
            Int(val) => format!("Integer ({})", val),
            Float(val) => format!("Floating-Point ({})", val),
            Bool(val) => format!("Boolean ({})", val),
            Decl { .. } => "Declaration".to_string(),
            Assign { .. } => "Assignment".to_string(),
            If { .. } => "If Statement".to_string(),
            Loop { .. } => "Loop".to_string(),
            While { .. } => "While Loop".to_string(),
            For { .. } => "For Loop".to_string(),
            Block(e) if e.is_empty() => "Empty Block".to_string(),
            Parens(_) => "Parentheses".to_string(),
            Block(_) => "Block".to_string(),
            Pair { .. } => "Pair".to_string(),
            Use { .. } => "Use".to_string(),
            Unknown => "<! Unknown !>".to_string(),
        }
    }

    fn color(&self) -> Option<AnsiColor> {
        use ast::NodeKind::*;
        let col = match &self.kind {
            // BinOp(..) | Comp(..) | Range(..) | UnOp(..) => AnsiColor::White,
            Use { .. } => AnsiColor::Blue,
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

    fn children(&'a self) -> PrettyChildren<'a> {
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
            Range { from, range, to } => pretty::Builder::new()
                .named("From", from)
                .named("Range Type", range)
                .named("To", to)
                .finish(),
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
            Use { path } => pretty::Builder::new().named("Path", *path).finish(),
            Parens(elems) | Block(elems) => elems.children(),
            Pair { lhs, rhs } => {
                pretty::Builder::new().named("Left", *lhs).named("Right", *rhs).finish()
            }
            Ident(..) | Int(..) | Float(..) | Bool(..) | Unknown => {
                pretty::Builder::empty()
            }
        }
    }
}

impl<'a, T: Symbolic> Pretty<'a> for T {
    fn title(&self) -> String {
        format!("{} ({})", self.name(), self.as_str())
    }

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&self) -> PrettyChildren<'a> {
        pretty::Builder::empty()
    }
}

impl<'a> Pretty<'a> for Vec<NodeRef<'a>> {
    fn title(&self) -> String {
        if self.is_empty() { "Empty List" } else { "List" }.to_string()
    }

    fn color(&self) -> Option<AnsiColor> {
        if self.is_empty() { Some(AnsiColor::Gray) } else { None }
    }

    fn children(&self) -> PrettyChildren<'a> {
        self.iter()
            .map(|node| PrettyChild::Unnamed { inner: *node as &dyn Pretty<'a> })
            .collect()
    }
}

impl<'a> Pretty<'a> for Option<NodeRef<'a>> {
    fn title(&self) -> String {
        match self {
            Some(inner) => format!("Some {}", inner.title()),
            None => "None".to_string(),
        }
    }

    fn color(&self) -> Option<AnsiColor> {
        if self.is_some() { None } else { Some(AnsiColor::Gray) }
    }

    fn children(&self) -> PrettyChildren<'a> {
        self.map_or_else(|| pretty::Builder::empty(), |inner| inner.children())
    }
}

/*use std::fmt;

use smallvec::SmallVec;

use crate::{
    ast::{self, binop::BinOpKind, symbol::Symbol},
    pretty::{self, Pretty, color::AnsiColor},
};

impl<'a> Pretty<'a> for ast::Node<'a> {
    fn color(&self) -> Option<AnsiColor> {
        use crate::ast::NodeKind::*;
        let col = match &self.kind {
            // BinOp(..) | Comp(..) | Range(..) | UnOp(..) => AnsiColor::White,
            Use { .. } => AnsiColor::Blue,
            KwBinOp { .. } | If { .. } | Loop { .. } | While { .. } | For { .. } => {
                AnsiColor::Purple
            }
            Access { .. } | Pair { .. } => AnsiColor::Yellow,
            Ident(_) => AnsiColor::Cyan,
            Decl { .. } | Assign { .. } => AnsiColor::Green,
            Block(e) if e.is_empty() => AnsiColor::Gray,
            Unknown => AnsiColor::Red,
            _ => return None,
        };
        Some(col)
    }

    fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ast::NodeKind::*;
        match &self.kind {
            Ident(name) => write!(f, "Identifier ({})", name),
            BinOp { .. } => f.write_str("Binary Operation"),
            KwBinOp { .. } => f.write_str("Binary Keyword Operation"),
            Comp { .. } => f.write_str("Comparison"),
            Range { .. } => f.write_str("Range"),
            UnOp { .. } => f.write_str("Unary Operation"),
            Access { .. } => f.write_str("Member Access"),
            Invoc { .. } => f.write_str("Invocation"),
            Int(val) => write!(f, "Integer ({})", val),
            Float(val) => write!(f, "Floating-Point Number ({})", val),
            Bool(val) => write!(f, "Boolean ({})", val),
            Decl { .. } => f.write_str("Declaration"),
            Assign { .. } => f.write_str("Assignment"),
            If { .. } => f.write_str("If Statement"),
            Loop { .. } => f.write_str("Loop"),
            While { .. } => f.write_str("While Loop"),
            For { .. } => f.write_str("For Loop"),
            Block(e) if e.is_empty() => f.write_str("Empty Block"),
            Parens(_) => f.write_str("Parentheses"),
            Block(_) => f.write_str("Block"),
            Pair { .. } => f.write_str("Pair"),
            Use { .. } => f.write_str("Use"),
            Unknown => f.write_str("<! Unknown !>"),
        }
    }

    fn children(&self) -> pretty::ChildNodes<'a> {
        use crate::ast::NodeKind::*;
        match &self.kind {
            BinOp { lhs, op, rhs } => pretty::Builder::new()
                .named("Left-hand Side", *lhs)
                .named("Operator", op)
                .named("Right-hand Side", *rhs)
                .finish(),
            /*KwBinOp { lhs, op, rhs } => pretty::Builder::new()
                .named("Left-hand Side", *lhs)
                .named("Operator", op.into_sym())
                .named("Right-hand Side", *rhs)
                .finish(),
            Comp { lhs, comp, rhs } => pretty::Builder::new()
                .named("Left-hand Side", *lhs)
                .named("Comparator", comp.into_sym())
                .named("Right-hand Side", *rhs)
                .finish(),
            Range { from, range, to } => pretty::Builder::new()
                .named("From", from)
                .named("Range Type", range.into_sym())
                .named("To", to)
                .finish(),
            UnOp { op, rhs } => pretty::Builder::new()
                .named("Operator", op.into_sym())
                .named("Right-hand Side", *rhs)
                .finish(),*/
            Access { parent, child } => pretty::Builder::new()
                .named("Parent", *parent)
                .named("Child", *child)
                .finish(),
            Invoc { callee, args } => pretty::Builder::new()
                .named("Callee", *callee)
                .named("Arguments", *args)
                .finish(),
            Decl { pat, val } => pretty::Builder::new()
                .named("Pattern", *pat)
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
            Parens(elems) => elems.children(),
            Block(elems) => elems.children(),
            Pair { lhs, rhs } => {
                pretty::Builder::new().named("Left", *lhs).named("Right", *rhs).finish()
            }
            _ => pretty::Builder::empty(),
        }
    }
}

impl<'a> Pretty<'a> for Symbol {
    fn color(&self) -> Option<AnsiColor> {
        todo!()
    }

    fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }

    fn children(&self) -> pretty::ChildNodes<'a> {
        todo!()
    }
}

impl<'a, T: Pretty<'a>> Pretty<'a> for Vec<&T> {
    fn color(&self) -> Option<AnsiColor> {
        todo!()
    }

    fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }

    fn children(&self) -> pretty::ChildNodes<'a> {
        todo!()
    }
}

impl<'a, T: Pretty<'a> + 'a, A> Pretty<'a> for SmallVec<A>
where
    A: smallvec::Array<Item = &'a T>,
{
    fn color(&self) -> Option<AnsiColor> {
        todo!()
    }

    fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }

    fn children(&self) -> pretty::ChildNodes<'a> {
        todo!()
    }
}

impl<'a, T: Pretty<'a>> Pretty<'a> for Option<&T> {
    fn color(&self) -> Option<AnsiColor> {
        todo!()
    }

    fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }

    fn children(&self) -> pretty::ChildNodes<'a> {
        todo!()
    }
}

impl<'a> Pretty<'a> for BinOpKind {
    fn color(&self) -> Option<AnsiColor> {
        todo!()
    }

    fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }

    fn children(&self) -> pretty::ChildNodes<'a> {
        todo!()
    }
}
*/
