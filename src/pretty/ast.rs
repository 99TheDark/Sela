use crate::{
    ast::{self, symbol::Symbol},
    pretty::{AnsiColor, Pretty, PrettyChild},
    prettyvec,
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
            KwBinOp { .. } => AnsiColor::Purple,
            Ident(_) => AnsiColor::Cyan,
            Decl { .. } => AnsiColor::Green,
            Unknown => AnsiColor::Red,
            _ => return None,
        };
        Some(col)
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
        use ast::NodeKind::*;
        match self.kind {
            BinOp { lhs, ref op, rhs } => {
                prettyvec![
                    ("Left-hand Side", lhs),
                    ("Operator", op),
                    ("Right-hand Side", rhs),
                ]
            }
            KwBinOp { lhs, ref op, rhs } => prettyvec![
                ("Left-hand Side", lhs),
                ("Operator", op),
                ("Right-hand Side", rhs),
            ],
            Comp { lhs, ref comp, rhs } => prettyvec![
                ("Left-hand Side", lhs),
                ("Comparator", comp),
                ("Right-hand Side", rhs),
            ],
            Range {
                ref from,
                ref range,
                ref to,
            } => {
                prettyvec![("From", from), ("Range Type", range), ("To", to)]
            }
            UnOp { ref op, rhs } => {
                prettyvec![("Operator", op), ("Right-hand Side", rhs)]
            }
            Decl { pat, val } => prettyvec![("Pattern", pat), ("Value", val)],
            If {
                cond,
                body,
                ref fallback,
            } => prettyvec![
                ("Condition", cond),
                ("Then Body", body),
                ("Else Body", fallback)
            ],
            Block(ref elems) => elems.children(),
            _ => prettyvec![],
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
        prettyvec![]
    }
}

impl<'a> Pretty for Vec<&'a ast::Node<'a>> {
    fn title(&self) -> String {
        "List".to_string()
    }

    fn children(&self) -> SmallVec<[PrettyChild<'_>; 3]> {
        self.iter()
            .map(|node| PrettyChild::Unnamed(*node as &dyn Pretty))
            .collect()
    }
}

impl<'a> Pretty for Option<&'a ast::Node<'a>> {
    fn title(&self) -> String {
        "Optional Node".to_string()
    }
}
