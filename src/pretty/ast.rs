use crate::{
    ast::{self, symbol::Symbol},
    pretty::{self, AnsiColor, Pretty, PrettyChild},
};
use smallvec::SmallVec;

impl<'a> Pretty for ast::Node<'a> {
    fn title(&self) -> String {
        use ast::NodeKind::*;
        match &self.kind {
            Ident(name) => format!("Identifier ({})", name),
            BinOp { .. } => "Binary Operation".to_string(),
            KwBinOp { .. } => "Binary Keyword Operation".to_string(),
            Comp { .. } => "Comparison".to_string(),
            Range { .. } => "Range".to_string(),
            UnOp { .. } => "Unary Operation".to_string(),
            Int(val) => format!("Integer ({})", val),
            Bool(val) => format!("Boolean ({})", val),
            Decl { .. } => "Declaration".to_string(),
            If { .. } => "If Statement".to_string(),
            Block(_) => "Block".to_string(),
            Unknown => "<! Unknown !>".to_string(),
        }
    }

    fn color(&self) -> Option<AnsiColor> {
        use ast::NodeKind::*;
        let col = match self.kind {
            // BinOp(..) | Comp(..) | Range(..) | UnOp(..) => AnsiColor::White,
            KwBinOp { .. } | If { .. } => AnsiColor::Purple,
            Ident(_) => AnsiColor::Cyan,
            Decl { .. } => AnsiColor::Green,
            Unknown => AnsiColor::Red,
            _ => return None,
        };
        Some(col)
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
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
            Decl { pat, val } => pretty::Builder::new()
                .named("Pattern", *pat)
                .named("Value", *val)
                .finish(),
            If { cond, body, fallback } => pretty::Builder::new()
                .named("Condition", *cond)
                .named("Then Body", *body)
                .named("Else Body", fallback)
                .finish(),
            Block(elems) => elems.children(),
            _ => pretty::Builder::empty(),
        }
    }
}

impl<T: Symbol> Pretty for T {
    fn title(&self) -> String {
        format!("{} ({})", self.name(), self.as_str())
    }

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
        pretty::Builder::empty()
    }
}

impl<'a> Pretty for Vec<&'a ast::Node<'a>> {
    fn title(&self) -> String {
        "List".to_string()
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
        self.iter()
            .map(|node| PrettyChild::Unnamed { inner: *node as &dyn Pretty })
            .collect()
    }
}

impl<'a> Pretty for Option<&'a ast::Node<'a>> {
    fn title(&self) -> String {
        match self {
            Some(inner) => format!("Some {}", inner.title()),
            None => "None".to_string(),
        }
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
        self.map_or_else(|| pretty::Builder::empty(), |inner| inner.children())
    }
}
